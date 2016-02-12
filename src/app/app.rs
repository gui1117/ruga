use util::direction::Direction;
use world::{ World, BodyTrait };
use world::body::character::CharacterTrait;
use world::body::character::GunType;
use world::body::character;
use maze::generate_kruskal;
use sound_manager::SoundManager;
use graphic_manager::GraphicManager;

pub struct App {
    pub world: World,
    pub quit: bool,
    pub player_dir: Vec<Direction>,
    pub window_size: [f64;2],
    pub sound_manager: SoundManager,
    pub graphic_manager: GraphicManager,
//    pub debug: usize,
//    pub debug2: f64,
}

const ZOOM: f64 = 8.;

impl App {
    pub fn new(width: f64, height: f64) -> App {
        let app = App {
            world: generate_kruskal(),
            quit: false,
            window_size: [width,height],
            player_dir: vec![],
            sound_manager: SoundManager::new(),
            graphic_manager: GraphicManager::new(),
//            debug: 0,
//            debug2: 0.,
        };

        app
    }

    pub fn render(&mut self) {
        //use graphics::*;

        //{
        //    let player = self.world.characters[0].borrow();
        //    self.camera.x = player.x();
        //    self.camera.y = player.y();
        //}

        //let listener = {
        //    let character = self.world.characters[0].borrow();
        //    [character.x(),character.y()]
        //};
        //self.sound_manager.set_listener(listener);

        //const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        //self.gl.draw(args.viewport(), |_, gl| {
        //    clear(BLACK, gl);
        //});

        //self.world.render_debug(&args.viewport(),&self.camera,&mut self.gl,&mut self.sound_manager);

//        if !false {
//            self.debug += 1;
//            self.debug2 += args.ext_dt / 10.;
//            if self.debug >= 10 {
//                println!("{}",1./self.debug2);
//                self.debug2 = 0.;
//                self.debug = 0;
//            }
//        }
    }

    pub fn update(&mut self, dt: f64) {
        self.world.update(dt);
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

