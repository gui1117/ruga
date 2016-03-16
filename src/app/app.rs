use utils::Direction;
use mazes::generate_kruskal;
use sound_manager::SoundManager;
use effect_manager::EffectManager;
use frame_manager::{
    FrameManager,
    Assets,
};
use event_loop::{
    RenderArgs,
    UpdateArgs,
};
use entities::{Character,CharacterManager};
use world::World;
use glium::backend::glutin_backend::GlutinFacade;

use std::rc::Rc;
use std::cell::RefCell;

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
}

const ZOOM: f64 = 0.05;

impl App {
    pub fn new(facade: &GlutinFacade) -> App {
        let window = facade.get_window().unwrap();
        let (width,height) = window.get_inner_size_pixels().unwrap();

        let (world,player) = generate_kruskal();

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
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
        let (x,y) = self.player.position();
        let mut frame_manager = FrameManager::new(&self.frame_assets,args.frame,args.ext_dt,x,y,self.zoom);

        self.sound_manager.set_listener([x,y]);

        self.effect_manager.render(&mut frame_manager, &mut self.sound_manager);
        self.world.render(&mut frame_manager);
        frame_manager.finish();
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.world.update(args.dt,&mut self.effect_manager);
    }
}

