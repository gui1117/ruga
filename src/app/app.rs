use util::direction::Direction;
use opengl_graphics::GlGraphics;
use world::{ World, Camera, BodyTrait };
use world::body::character::CharacterTrait;
use piston::input::{ 
    RenderArgs, 
    UpdateArgs, 
    UpdateEvent,
};
use maze::generate_kruskal;

pub struct App {
    pub gl: GlGraphics,
    pub world: World,
    pub quit: bool,
    pub camera: Camera,
    pub player_dir: Vec<Direction>,
    pub window_size: [f64;2],
//    pub debug: usize,
//    pub debug2: f64,
}

impl App {
    pub fn new(gl: GlGraphics, width: f64, height: f64) -> App {
        let app = App {
            gl: gl,
            world: generate_kruskal(),
            quit: false,
            window_size: [width,height],
            player_dir: vec![],
            camera: Camera::new(0.,0., width, height, 1.),
//            debug: 0,
//            debug2: 0.,
        };

        app
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        {
            let player = self.world.characters[0].clone();
            self.camera.x = player.x();
            self.camera.y = player.y();
        }

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_, gl| {
            clear(BLACK, gl);
        });

        self.world.render_debug(&args.viewport(),&self.camera,&mut self.gl);

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

    pub fn update(&mut self, args: &UpdateArgs) {
        self.world.update(args.dt);
    }

    pub fn player_aim(&self) -> f64 {
        self.world.characters[0].aim()
    }

    pub fn set_player_aim(&mut self, aim: f64) {
        self.world.characters[0].set_aim(aim);
    }

    pub fn player_velocity(&self) -> f64 {
        self.world.characters[0].velocity()
    }

    pub fn set_player_velocity(&mut self, v: f64) {
        self.world.characters[0].set_velocity(v);
    }

    pub fn player_angle(&self) -> f64 {
        self.world.characters[0].angle()
    }

    pub fn set_player_angle(&mut self, a: f64) {
        self.world.characters[0].set_angle(a);
    }

    pub fn set_player_shoot(&mut self) {
        self.world.characters[0].gun_shoot();
    }

    //pub fn set_player_launch_grenade(&mut self) {
    //	if let Some(id) = self.player_id {
    //		GrenadeLauncher::shoot(&mut self.world, id);
    //	}
    //}
}

