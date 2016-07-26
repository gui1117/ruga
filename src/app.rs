use graphics;
use specs;
use utils::Direction;
use event_loop;
use config;
use glium;
use specs::Join;
use levels;
use systems::*;
use components::*;
use std::sync::mpsc;

pub struct Graphic {
    color: graphics::Color,
    layer: graphics::Layer,
}
impl Graphic {
    pub fn new(color: graphics::Color, layer: graphics::Layer) -> Self {
        Graphic {
            color: color,
            layer: layer,
        }
    }
}
impl specs::Component for Graphic {
    type Storage = specs::VecStorage<Self>;
}

pub enum Effect {
    Line {
        origin: [f32;2],
        length: f32,
        angle: f32,
        persistance: f32,
        thickness: f32,
        layer: graphics::Layer,
        color: graphics::Color,
    },
}
impl Effect {
    fn next(self,dt: f32) -> Option<Effect> {
        match self {
            Effect::Line { origin: o, length: le, angle: a, persistance: mut p, thickness: t, layer: la, color: c, }
            => {
                p -= dt;
                if p > 0. {
                    Some(Effect::Line { origin: o, length: le, angle: a, persistance: p, thickness: t, layer: la, color: c, })
                } else {
                    None
                }
            },
        }
    }
    fn draw(&self, frame: &mut graphics::Frame) {
        match self {
            &Effect::Line {
                origin: o,
                length: le,
                angle: a,
                persistance: _,
                thickness: t,
                layer: la,
                color: co,
            } => {
                frame.draw_line(o[0],o[1],a,le,t,la,co);
            },
        }
    }
}

pub enum Control {
    GotoLevel(levels::Level),
    ResetLevel,
}

#[derive(Clone)]
pub struct UpdateContext {
    pub effect_tx: mpsc::Sender<Effect>,
    pub control_tx: mpsc::Sender<Control>,
    pub dt: f64,
    pub master_entity: specs::Entity,
}

pub struct App {
    current_level: levels::Level,
    camera: graphics::Camera,
    graphics: graphics::Graphics,
    planner: specs::Planner<UpdateContext>,
    player_dir: Vec<Direction>,
    control_rx: mpsc::Receiver<Control>,
    control_tx: mpsc::Sender<Control>,
    effect_rx: mpsc::Receiver<Effect>,
    effect_storage: Vec<Effect>,
    effect_tx: mpsc::Sender<Effect>,
    master_entity: specs::Entity,
}

impl App {
    pub fn new<F: glium::backend::Facade>(facade: &F) -> Result<App,String> {
        // init graphics
        let graphics = try!(graphics::Graphics::new(facade, graphics::GraphicsSetting {
            colors: graphics::ColorsValue {
                base03: config.graphics.base03,
                base02: config.graphics.base02,
                base01: config.graphics.base01,
                base00: config.graphics.base00,
                base0: config.graphics.base0,
                base1: config.graphics.base1,
                base2: config.graphics.base2,
                base3: config.graphics.base3,
                yellow: config.graphics.yellow,
                orange: config.graphics.orange,
                red: config.graphics.red,
                magenta: config.graphics.magenta,
                violet: config.graphics.violet,
                blue: config.graphics.blue,
                cyan: config.graphics.cyan,
                green: config.graphics.green,
            },
            mode: match &*config.graphics.mode {
                "light" => graphics::Mode::Light,
                "dark" => graphics::Mode::Dark,
                _ => unreachable!(),
            },
            luminosity: config.graphics.luminosity,
            circle_precision: config.graphics.circle_precision,
            font_precision: config.graphics.font_precision,
            font_file: config.graphics.font_file.clone(),
            font_ratio: config.graphics.font_ratio,
        }).map_err(|e| format!("ERRROR: graphics init failed: {:#?}",e)));
        // init camera
        let camera = try!(graphics::Camera::new(facade, graphics::CameraSetting {
            zoom: config.camera.zoom
        }).map_err(|e| format!("ERROR: camera init failed: {:#?}",e)));

        // init world
        let mut world = specs::World::new();
        world.register::<PlayerControl>();
        world.register::<TowardPlayerControl>();
        world.register::<MonsterControl>();

        world.register::<PhysicState>();
        world.register::<PhysicForce>();
        world.register::<PhysicType>();
        world.register::<PhysicWorld>();
        world.register::<PhysicDynamic>();
        world.register::<PhysicStatic>();
        world.register::<PhysicTrigger>();
        world.register::<GridSquare>();

        world.register::<Graphic>();

        world.register::<Life>();
        world.register::<Killer>();
        world.register::<Ball>();

        world.register::<Portal>();

        // load level
        let master_entity = try!(levels::load(&levels::Level::Entry,&mut world)
                                 .map_err(|e| format!("ERROR: level load failed: {:#?}",e)));

        // init planner
        let mut planner = specs::Planner::new(world,config.general.number_of_thread);
        planner.add_system(PhysicSystem, "physic", 10);
        planner.add_system(PlayerSystem, "player", 5);
        planner.add_system(MonsterSystem, "monster", 5);
        planner.add_system(TowardPlayerSystem, "toward_player", 5);
        planner.add_system(KillerSystem, "killer", 5);
        planner.add_system(BallSystem, "ball", 5);
        planner.add_system(PortalSystem, "portal", 5);
        planner.add_system(LifeSystem, "life", 1);

        let (effect_tx, effect_rx) = mpsc::channel();
        let (control_tx, control_rx) = mpsc::channel();

        Ok(App {
            current_level: levels::Level::Entry,
            effect_storage: Vec::new(),
            camera: camera,
            graphics: graphics,
            planner: planner,
            player_dir: vec!(),
            master_entity: master_entity,
            effect_rx: effect_rx,
            effect_tx: effect_tx,
            control_rx: control_rx,
            control_tx: control_tx,
        })
    }
    pub fn update(&mut self, args: event_loop::UpdateArgs) {
        let context = UpdateContext {
            dt: args.dt,
            master_entity: self.master_entity,
            effect_tx: self.effect_tx.clone(),
            control_tx: self.control_tx.clone(),
        };

        self.planner.dispatch(context);
        self.planner.wait();

        while let Ok(control) = self.control_rx.try_recv() {
            match control {
                Control::GotoLevel(level) => self.goto_level(level),
                Control::ResetLevel => {
                    let level = self.current_level.clone();
                    self.goto_level(level);
                }
            }
        }
    }
    pub fn goto_level(&mut self, level: levels::Level) {
        //TODO keep the velocity and acceleration if set on argument
        while let Ok(_) = self.control_rx.try_recv() {}
        while let Ok(_) = self.effect_rx.try_recv() {}

        self.master_entity = match levels::load(&level,self.planner.mut_world()) {
            Err(e) => panic!(format!("ERROR: level load failed: {:#?}",e)),
            Ok(m) => m,
        };

        self.current_level = level;
        let mut player_dir = vec!();
        player_dir.append(&mut self.player_dir);
        for k in player_dir {
            match k {
                Direction::Up => self.key_pressed(config.keys.up[0]),
                Direction::Down => self.key_pressed(config.keys.down[0]),
                Direction::Right => self.key_pressed(config.keys.right[0]),
                Direction::Left => self.key_pressed(config.keys.left[0]),
            }
        }
    }
    pub fn render(&mut self, args: event_loop::RenderArgs) {
        let dt = 1. / config.event_loop.max_fps as f32;
        let world = self.planner.mut_world();

        // update camera
        {
            let characters = world.read::<PlayerControl>();
            let states = world.read::<PhysicState>();
            for (_, state) in (&characters, &states).iter() {
                self.camera.x = state.position[0];
                self.camera.y = state.position[1];
            }
        }

        let mut frame = graphics::Frame::new(&self.graphics, args.frame, &self.camera);

        // draw entities
        {
            let states = world.read::<PhysicState>();
            let types = world.read::<PhysicType>();
            let graphics = world.read::<Graphic>();
            let squares = world.read::<GridSquare>();

            for (square, graphic) in (&squares, &graphics).iter() {
                let p = square.position;
                frame.draw_square(p[0],p[1],0.5,graphic.layer,graphic.color);
            }

            for (state, typ, graphic) in (&states, &types, &graphics).iter() {
                let x = state.position[0];
                let y = state.position[1];
                match typ.shape {
                    Shape::Circle(radius) => frame.draw_circle(x,y,radius,graphic.layer,graphic.color),
                    Shape::Square(radius) => frame.draw_square(x,y,radius,graphic.layer,graphic.color),
                }
            }
        }


        // draw effects
        for effect in &self.effect_storage {
            effect.draw(&mut frame);
        }

        let old_effect_storage = self.effect_storage.drain(..).collect::<Vec<Effect>>();;
        for effect in old_effect_storage {
            if let Some(effect) = effect.next(dt) {
                self.effect_storage.push(effect)
            }
        }

        while let Ok(effect) = self.effect_rx.try_recv() {
            effect.draw(&mut frame);
            if let Some(effect) = effect.next(dt) {
                self.effect_storage.push(effect);
            }
        }

        frame.finish().unwrap();
    }
    pub fn key_pressed(&mut self, key: u8) {
        if config.keys.up.contains(&key) {
            if !self.player_dir.contains(&Direction::Up) {
                self.player_dir.push(Direction::Up);
                self.update_player_direction();
            }
        } else if config.keys.down.contains(&key) {
            if !self.player_dir.contains(&Direction::Down) {
                self.player_dir.push(Direction::Down);
                self.update_player_direction();
            }
        } else if config.keys.left.contains(&key) {
            if !self.player_dir.contains(&Direction::Left) {
                self.player_dir.push(Direction::Left);
                self.update_player_direction();
            }
        } else if config.keys.right.contains(&key) {
            if !self.player_dir.contains(&Direction::Right) {
                self.player_dir.push(Direction::Right);
                self.update_player_direction();
            }
        }
    }
    pub fn key_released(&mut self, key: u8) {
        if config.keys.up.contains(&key) {
            self.player_dir.retain(|dir| &Direction::Up != dir);
            self.update_player_direction();
        } else if config.keys.down.contains(&key) {
            self.player_dir.retain(|dir| &Direction::Down != dir);
            self.update_player_direction();
        } else if config.keys.left.contains(&key) {
            self.player_dir.retain(|dir| &Direction::Left != dir);
            self.update_player_direction();
        } else if config.keys.right.contains(&key) {
            self.player_dir.retain(|dir| &Direction::Right != dir);
            self.update_player_direction();
        }
    }
    fn update_player_direction(&mut self) {
        use std::f32::consts::PI;

        let world = self.planner.mut_world();

        if let Some(dir) = self.player_dir.last() {

            let mut last_perpendicular: Option<&Direction> = None;
            for d in &self.player_dir {
                if d.perpendicular(dir) {
                    last_perpendicular = Some(d);
                }
            }

            let angle = match dir {
                &Direction::Up => {
                    match last_perpendicular {
                        Some(&Direction::Left) => 3.*PI/4.,
                        Some(&Direction::Right) => PI/4.,
                        _ => PI/2.,
                    }
                },
                &Direction::Down => {
                    match last_perpendicular {
                        Some(&Direction::Left) => -3.*PI/4.,
                        Some(&Direction::Right) => -PI/4.,
                        _ => -PI/2.,
                    }
                },
                &Direction::Right => {
                    match last_perpendicular {
                        Some(&Direction::Down) => -PI/4.,
                        Some(&Direction::Up) => PI/4.,
                        _ => 0.,
                    }
                },
                &Direction::Left => {
                    match last_perpendicular {
                        Some(&Direction::Down) => -3.*PI/4.,
                        Some(&Direction::Up) => 3.*PI/4.,
                        _ => PI,
                    }
                },
            };

            let characters = world.read::<PlayerControl>();
            let mut forces = world.write::<PhysicForce>();
            for (_, force) in (&characters, &mut forces).iter() {
                force.direction = angle;
                force.intensity = 1.;
            }
        } else {
            let characters = world.read::<PlayerControl>();
            let mut forces = world.write::<PhysicForce>();
            for (_, force) in (&characters, &mut forces).iter() {
                force.intensity = 0.;
            }
        }
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.camera.ratio = width as f32 / height as f32;
    }
}

