use graphics;
use specs;
use Direction;
use event_loop;
use config;
use key;
use glium::glutin::MouseButton;
use glium;
use specs::Join;
use levels;
use systems::*;
use components::*;
use std::sync::mpsc;

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

struct Cursor {
    position: [f32;2],
    inner_radius: f32,
    outer_radius: f32,
    color: graphics::Color,
}

#[derive(Clone)]
pub struct UpdateContext {
    pub effect_tx: mpsc::Sender<Effect>,
    pub dt: f64,
    pub master_entity: specs::Entity,
}

pub struct App {
    camera: graphics::Camera,
    graphics: graphics::Graphics,
    cursor: Cursor,
    planner: specs::Planner<UpdateContext>,
    player_dir: Vec<Direction>,
    /// 0: shoot one
    /// 1: shoot lots
    player_state: [bool;2],
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
        world.register::<PhysicState>();
        world.register::<PhysicStatic>();
        world.register::<PhysicDynamic>();
        world.register::<PhysicKinetic>();
        world.register::<PhysicType>();
        world.register::<PhysicForce>();
        world.register::<PlayerControl>();
        world.register::<Color>();
        world.register::<Rifle>();
        world.register::<PhysicWorld>();
        world.register::<Life>();

        // load level
        let master_entity = try!(levels::load(config.levels.first_level.clone(),&world)
                                 .map_err(|e| format!("ERROR: level load failed: {:#?}",e)));

        // init planner
        let mut planner = specs::Planner::new(world,config.general.number_of_thread);
        let weapons_system = WeaponSystem;
        let physic_system = PhysicSystem;
        planner.add_system(weapons_system, "weapons", 10);
        planner.add_system(physic_system, "physic", 30);

        let cursor = Cursor {
            position: [0.,0.],
            inner_radius: config.cursor.inner_radius,
            outer_radius: config.cursor.outer_radius,
            color: match &*config.cursor.color {
                "base5" => graphics::Color::Base5,
                "base4" => graphics::Color::Base4,
                "base3" => graphics::Color::Base3,
                "base2" => graphics::Color::Base2,
                "base1" => graphics::Color::Base1,
                "yellow" => graphics::Color::Yellow,
                "orange" => graphics::Color::Orange,
                "red" => graphics::Color::Red,
                "magenta" => graphics::Color::Magenta,
                "violet" => graphics::Color::Violet,
                "blue" => graphics::Color::Blue,
                "cyan" => graphics::Color::Cyan,
                "green" => graphics::Color::Green,
                _ => unreachable!(),
            },
        };

        let (effect_tx, effect_rx) = mpsc::channel();

        Ok(App {
            effect_storage: Vec::new(),
            camera: camera,
            graphics: graphics,
            cursor: cursor,
            planner: planner,
            player_dir: vec!(),
            player_state: [false,false],
            master_entity: master_entity,
            effect_rx: effect_rx,
            effect_tx: effect_tx,
        })
    }
    pub fn update(&mut self, args: event_loop::UpdateArgs) {
        let context = UpdateContext {
            dt: args.dt,
            master_entity: self.master_entity,
            effect_tx: self.effect_tx.clone(),
        };

        self.planner.dispatch(context);
        self.planner.wait();
    }
    pub fn render(&mut self, args: event_loop::RenderArgs) {
        let dt = 1. / config.event_loop.max_fps as f32;

        // update camera
        {
            let characters = self.planner.world.read::<PlayerControl>();
            let states = self.planner.world.read::<PhysicState>();
            for (_, state) in (&characters, &states).iter() {
                self.camera.x = state.position[0];
                self.camera.y = state.position[1];
            }
        }

        let mut frame = graphics::Frame::new(&self.graphics, args.frame, &self.camera);

        // draw entities
        {
            let states = self.planner.world.read::<PhysicState>();
            let types = self.planner.world.read::<PhysicType>();
            let colors = self.planner.world.read::<graphics::Color>();

            for (state, typ, color) in (&states, &types, &colors).iter() {
                let x = state.position[0];
                let y = state.position[1];
                match typ.shape {
                    Shape::Circle(radius) => frame.draw_circle(x,y,radius,graphics::Layer::Middle,*color),
                    Shape::Square(radius) => frame.draw_square(x,y,radius,graphics::Layer::Middle,*color),
                }
            }
        }

        // draw cursor
        {
            let cursor = self.cursor_absolute_position();

            frame.draw_rectangle(
                cursor[0],
                cursor[1],
                self.cursor.outer_radius,
                self.cursor.inner_radius,
                graphics::Layer::Ceil,
                self.cursor.color);

            frame.draw_rectangle(
                cursor[0],
                cursor[1],
                self.cursor.inner_radius,
                self.cursor.outer_radius,
                graphics::Layer::Ceil,
                self.cursor.color);
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
        match key {
            key::Z => {
                if !self.player_dir.contains(&Direction::Up) {
                    self.player_dir.push(Direction::Up);
                    self.update_player_direction();
                }
            },
            key::S => {
                if !self.player_dir.contains(&Direction::Down) {
                    self.player_dir.push(Direction::Down);
                    self.update_player_direction();
                }
            },
            key::Q => {
                if !self.player_dir.contains(&Direction::Left) {
                    self.player_dir.push(Direction::Left);
                    self.update_player_direction();
                }
            },
            key::D => {
                if !self.player_dir.contains(&Direction::Right) {
                    self.player_dir.push(Direction::Right);
                    self.update_player_direction();
                }
            },
            _ => (),
        }
    }
    pub fn key_released(&mut self, key: u8) {
        match key {
            key::Z => {
                self.player_dir.retain(|dir| &Direction::Up != dir);
                self.update_player_direction();
            }
            key::S => {
                self.player_dir.retain(|dir| &Direction::Down != dir);
                self.update_player_direction();
            }
            key::Q => {
                self.player_dir.retain(|dir| &Direction::Left != dir);
                self.update_player_direction();
            }
            key::D => {
                self.player_dir.retain(|dir| &Direction::Right != dir);
                self.update_player_direction();
            }
            _ => (),
        }
    }
    fn cursor_relative_position(&self) -> [f32;2] {
        [self.cursor.position[0]/self.camera.zoom, self.cursor.position[1]/self.camera.zoom/self.camera.ratio]
    }
    fn cursor_absolute_position(&self) -> [f32;2] {
        let rel = self.cursor_relative_position();
        [rel[0] + self.camera.x, rel[1] + self.camera.y]
    }
    fn update_rifle_state(&mut self) {
        let state = if self.player_state[1] && self.player_state[0] {
            RifleState::ShootLotsOrOne
        } else if self.player_state[1] {
            RifleState::ShootLots
        } else if self.player_state[0] {
            RifleState::ShootOne
        } else {
            RifleState::Rest
        };

        let characters = self.planner.world.read::<PlayerControl>();
        let mut rifles = self.planner.world.write::<Rifle>();
        for (_,mut rifle) in (&characters, &mut rifles).iter() {
            rifle.state = state
        }
    }
    pub fn mouse_pressed(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.player_state[0] = true;
                self.update_rifle_state();
            },
            MouseButton::Right => {
                self.player_state[1] = true;
                self.update_rifle_state();
            },
            _ => (),
        }
    }
    pub fn mouse_released(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.player_state[0] = false;
                self.update_rifle_state();
            },
            MouseButton::Right => {
                self.player_state[1] = false;
                self.update_rifle_state();
            },
            _ => (),
        }
    }
    pub fn mouse_moved(&mut self, x: f32, y: f32) {
        self.cursor.position = [x,y];
        let characters = self.planner.world.read::<PlayerControl>();
        let mut rifles = self.planner.world.write::<Rifle>();
        for (_, rifle) in (&characters, &mut rifles).iter() {
            let cursor = self.cursor_relative_position();
            rifle.aim = cursor[1].atan2(cursor[0]);
        }
    }
    fn update_player_direction(&mut self) {
        use std::f32::consts::PI;

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

            let characters = self.planner.world.read::<PlayerControl>();
            let mut forces = self.planner.world.write::<PhysicForce>();
            for (_, force) in (&characters, &mut forces).iter() {
                force.direction = angle;
                force.intensity = 1.;
            }
        } else {
            let characters = self.planner.world.read::<PlayerControl>();
            let mut forces = self.planner.world.write::<PhysicForce>();
            for (_, force) in (&characters, &mut forces).iter() {
                force.intensity = 0.;
            }
        }
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.camera.ratio = width as f32 / height as f32;
    }
}

