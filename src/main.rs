extern crate baal;
extern crate graphics;
extern crate glium;
extern crate hlua;
extern crate specs;
#[macro_use] extern crate yaml_utils;
extern crate yaml_rust;
extern crate time;

mod levels;
mod event_loop;
mod window;
pub mod control;
pub mod physic;
pub mod entities;
pub mod utils;

use glium::glutin::ElementState;
use glium::glutin::Event as InputEvent;
use glium::glutin::MouseButton;
use std::time::Duration;
use std::thread;
use event_loop::{
    Events,
    Event,
};
use yaml_utils::FromYaml;
use yaml_rust::yaml;
use specs::Join;
use utils::Direction;
use utils::key;

const NUMBER_OF_THREADS: usize = 4;

#[derive(Clone)]
pub struct UpdateContext {
    pub dt: f64
}

struct App {
    camera: graphics::Camera,
    graphics: graphics::Graphics,
    entities: entities::Entities,
    planner: specs::Planner<UpdateContext>,
    player_dir: Vec<Direction>,
}

impl App {
    fn update(&mut self, args: event_loop::UpdateArgs) {
        let context = UpdateContext {
            dt: args.dt,
        };

        self.planner.dispatch(context);
    }
    fn render(&mut self, args: event_loop::RenderArgs) {
        // update camera
        {
            let characters = self.planner.world.read::<control::PlayerControl>();
            let states = self.planner.world.read::<physic::PhysicState>();
            for (_, state) in (&characters, &states).iter() {
                self.camera.x = state.position[0];
                self.camera.y = state.position[1];
            }
        }

        let mut frame = graphics::Frame::new(&self.graphics, args.frame, &self.camera);

        // draw entities
        {
            let states = self.planner.world.read::<physic::PhysicState>();
            let types = self.planner.world.read::<physic::PhysicType>();
            let colors = self.planner.world.read::<graphics::Color>();

            for (state, typ, color) in (&states, &types, &colors).iter() {
                let x = state.position[0];
                let y = state.position[1];
                match typ.shape {
                    physic::Shape::Circle(radius) => frame.draw_circle(x,y,radius,graphics::Layer::Middle,*color),
                    physic::Shape::Square(radius) => frame.draw_square(x,y,radius,graphics::Layer::Middle,*color),
                }
            }
        }

        // draw grid
        // draw effects

        frame.finish().unwrap();
    }
    fn key_pressed(&mut self, key: u8) {
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
    fn key_released(&mut self, key: u8) {
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
    fn mouse_pressed(&mut self, _button: MouseButton) {}
    fn mouse_released(&mut self, _button: MouseButton) {}
    fn mouse_moved(&mut self, _x: i32, _y: i32) {}
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

            let characters = self.planner.world.read::<control::PlayerControl>();
            let mut forces = self.planner.world.write::<physic::PhysicForce>();
            for (_, force) in (&characters, &mut forces).iter() {
                force.direction = angle;
                force.intensity = 1.;
            }
        } else {
            let characters = self.planner.world.read::<control::PlayerControl>();
            let mut forces = self.planner.world.write::<physic::PhysicForce>();
            for (_, force) in (&characters, &mut forces).iter() {
                force.intensity = 0.;
            }
        }
    }
}

fn main() {
    let mut config = yaml_utils::unify(std::path::Path::new("config")).unwrap();

    // init baal
    let audio_config = config.remove(&yaml::Yaml::String("audio".into())).unwrap();
    let baal_setting = baal::Setting::from_yaml(&audio_config).unwrap();
    baal::init(&baal_setting).unwrap();

    // init window
    let window_config = config.remove(&yaml::Yaml::String("window".into())).unwrap();
    let window_setting = window::Setting::from_yaml(&window_config).unwrap();
    let mut window = window::create(&window_setting).unwrap();

    // init graphics
    let graphics_config = config.remove(&yaml::Yaml::String("graphics".into())).unwrap();
    let graphics_setting = graphics::GraphicsSetting::from_yaml(&graphics_config).unwrap();
    let graphics = graphics::Graphics::new(&window, graphics_setting).unwrap();

    // init camera
    let camera_config = config.remove(&yaml::Yaml::String("camera".into())).unwrap();
    let camera_setting = graphics::CameraSetting::from_yaml(&camera_config).unwrap();
    let camera = graphics::Camera::new(&window, camera_setting).unwrap();

    // init entities
    // let entities_config = config.remove(&yaml::Yaml::String("entities".into())).unwrap();
    // let entities_setting = entities::EntitiesSetting::from_yaml(entities_config).unwrap();
    let entities = entities::Entities::new();

    // init world
    let mut world = specs::World::new();
    world.register::<physic::PhysicState>();
    world.register::<physic::PhysicType>();
    world.register::<physic::PhysicForce>();
    world.register::<control::PlayerControl>();
    world.register::<graphics::Color>();

    // load level
    levels::load("toto".into(),&mut world,&entities);

    // init planner
    let planner = specs::Planner::new(world,NUMBER_OF_THREADS);

    // init event loop
    let event_loop_config = config.get(&yaml::Yaml::String("event_loop".into())).unwrap();
    let event_loop_setting = event_loop::Setting::from_yaml(event_loop_config).unwrap();
    let mut window_events = window.events(&event_loop_setting);

    let mut app = App {
        camera: camera,
        graphics: graphics,
        entities: entities,
        planner: planner,
        player_dir: vec!(),
    };

    // game loop
    while let Some(event) = window_events.next(&mut window) {
        match event {
            Event::Update(args) => app.update(args),
            Event::Render(args) => app.render(args),
            Event::Input(InputEvent::Closed) => break,
            Event::Input(InputEvent::KeyboardInput(state,keycode,_)) => {
                if state == ElementState::Pressed {
                    app.key_pressed(keycode);
                } else {
                    app.key_released(keycode);
                }
            },
            Event::Input(InputEvent::MouseInput(state,button)) => {
                if state == ElementState::Pressed {
                    app.mouse_pressed(button);
                } else {
                    app.mouse_released(button);
                }
            },
            Event::Input(InputEvent::MouseMoved(x,y)) => {
                app.mouse_moved(x,y);
            },
            Event::Input(_) => (),
            Event::Idle(args) => thread::sleep(Duration::from_millis(args.dt as u64)),
        }
    }
}

