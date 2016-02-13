use util::direction::Direction;
use world::{ World, BodyTrait };
use world::body::character::CharacterTrait;
use world::body::character::GunType;
use world::body::character;
use maze::generate_kruskal;
use sound_manager::SoundManager;
use frame_manager::{
    FrameManager,
    Assets,
};
use event_loop::{
    RenderArgs,
    UpdateArgs,
};

pub struct App {
    pub world: World,
    pub quit: bool,
    pub player_dir: Vec<Direction>,
    pub window_size: [u32;2],
    pub sound_manager: SoundManager,
    pub zoom: f64,
    pub frame_assets: Assets,
}

const ZOOM: f64 = 8.;

impl App {
    pub fn new(width: u32, height: u32) -> App {
        let app = App {
            world: generate_kruskal(),
            quit: false,
            window_size: [width,height],
            player_dir: vec![],
            sound_manager: SoundManager::new(),
            zoom: ZOOM,
            frame_assets: Assets::new(),
        };

        app
    }

    pub fn render(&mut self, args: RenderArgs) {
        let (x,y) = {
            let player = self.world.characters[0].borrow();
            (player.x(), player.y())
        };
        let mut frame_manager = FrameManager::new(&self.frame_assets,args.frame,args.ext_dt,x,y,self.zoom);

        let listener = {
            let character = self.world.characters[0].borrow();
            [character.x(),character.y()]
        };
        self.sound_manager.set_listener(listener);

        //const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        //self.gl.draw(args.viewport(), |_, gl| {
        //    clear(BLACK, gl);
        //});

        self.world.render(&mut frame_manager, &mut self.sound_manager);
        frame_manager.finish();
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.world.update(args.dt);
    }

    pub fn player_aim(&self) -> f64 {
        self.world.characters[0].aim()
    }

    pub fn set_player_aim(&mut self, aim: f64) {
        self.world.characters[0].set_aim(aim);
    }

    pub fn player_velocity(&self) -> f64 {
        self.world.characters[0].borrow().velocity()
    }

    pub fn set_player_velocity(&mut self, v: f64) {
        let v = v.min(1.);
        self.world.characters[0].borrow_mut().set_velocity(v*character::VELOCITY);
    }

    pub fn player_angle(&self) -> f64 {
        self.world.characters[0].borrow().angle()
    }

    pub fn set_player_angle(&mut self, a: f64) {
        self.world.characters[0].borrow_mut().set_angle(a);
    }

    pub fn set_player_attack_sword(&mut self) {
        let character = &*self.world.characters[0];
        character.do_sword_attack(&self.world.batch);
    }

    pub fn set_player_shoot(&mut self, shoot: bool) {
        self.world.characters[0].set_gun_shoot(shoot);
    }

    pub fn set_player_next_gun(&mut self, gun_type: GunType) {
        self.world.characters[0].set_next_gun_type(gun_type);
    }

    pub fn set_player_launch_grenade(&mut self) {
        let character = &*self.world.characters[0].clone();
        character.launch_grenade(&mut self.world);
    }
}

