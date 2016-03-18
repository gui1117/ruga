use utils::Direction;
use mazes::kruskal;
use sound_manager::SoundManager;
use effect_manager::EffectManager;
use frame_manager::{
    FrameManager,
    Assets,
    color,
};
use event_loop::{
    RenderArgs,
    UpdateArgs,
};
use entities::{Character,CharacterManager};
use world::{World, EntityCell};
use glium::backend::glutin_backend::GlutinFacade;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Rem;

pub struct App {
    pub world: World,
    pub quit: bool,
    pub player_dir: Vec<Direction>,
    pub player: Rc<RefCell<Character>>,
    pub window_size: [u32;2],
    pub sound_manager: SoundManager,
    pub effect_manager: EffectManager,
    pub zoom: f64,
    pub frame_assets: Assets,
    pub animation_state: usize,
    pub animation_state_counter: f64,
}

const ZOOM: f64 = 0.05;
const ANIMATION_RATE: f64 = 0.1;

impl App {
    pub fn new(facade: &GlutinFacade) -> App {
        let window = facade.get_window().unwrap();
        let (width,height) = window.get_inner_size_pixels().unwrap();

        let (world,player) = kruskal::generate();

        App {
            world: world,
            window_size: [width,height],
            player_dir: vec![],
            player: player,
            sound_manager: SoundManager::new(),
            zoom: ZOOM,
            frame_assets: Assets::new(facade),
            effect_manager: EffectManager::new(),
            quit: false,
            animation_state: 0,
            animation_state_counter: 0.,
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
        let (x,y) = self.player.position();
        let mut frame_manager = FrameManager::new(&self.frame_assets,args.frame,args.ext_dt,x,y,self.zoom,self.animation_state);

        self.sound_manager.set_listener([x,y]);

        self.effect_manager.render(&mut frame_manager, &mut self.sound_manager);
        self.world.render(&mut frame_manager);
        let player = self.player.clone() as Rc<EntityCell>;
        let life = player.borrow().body().life as i32;
        let dl = 1./15.;
        for i in 0..life {
            frame_manager.draw_interface_rectangle(color::BLACK,dl*(i as f64+1.)-1.,0.95,0.01,0.03);
        }
        frame_manager.finish();
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.animation_state_counter += args.dt;
        if self.animation_state_counter >= ANIMATION_RATE {
            self.animation_state_counter = 0.;
            self.animation_state = (self.animation_state+1).rem(4)
        }
        let player = self.player.clone() as Rc<EntityCell>;
        kruskal::update(&*player, &mut self.world);
        self.world.update(args.dt,&mut self.effect_manager);
    }
}

