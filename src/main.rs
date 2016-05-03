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

use glium::glutin::ElementState;
use glium::glutin::Event as InputEvent;
use std::time::Duration;
use std::thread;
use event_loop::{
    Events,
    Event,
};
use yaml_utils::FromYaml;
use yaml_rust::yaml;
use specs::Join;

const NUMBER_OF_THREADS: usize = 4;

#[derive(Clone)]
struct UpdateContext;

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
    let mut camera = graphics::Camera::new(&window, camera_setting).unwrap();

    // init entities
    let entities_config = config.remove(&yaml::Yaml::String("entities".into())).unwrap();
    let entities_setting = entities::EntitiesSetting::from_yaml(entities_config).unwrap();
    let entities = entities::Entities::new(entities_setting);

    // init world
    let mut world = specs::World::new();
    world.register::<physic::PhysicState>();
    world.register::<physic::PhysicType>();
    world.register::<physic::PhysicForce>();
    world.register::<graphics::Color>();

    // load level
    levels::load("toto".into(),&mut world,&entities);

    // init planner
    let mut planner = specs::Planner::new(world,NUMBER_OF_THREADS);

    // init event loop
    let event_loop_config = config.get(&yaml::Yaml::String("event_loop".into())).unwrap();
    let event_loop_setting = event_loop::Setting::from_yaml(event_loop_config).unwrap();
    let mut window_events = window.events(&event_loop_setting);

    // game loop
    while let Some(event) = window_events.next(&mut window) {
        match event {
            Event::Update(_args) => {
                planner.dispatch(UpdateContext);
            },
            Event::Render(args) => {
                // update camera
                {
                    let characters = planner.world.read::<control::PlayerControl>();
                    let states = planner.world.read::<physic::PhysicState>();
                    for (_, state) in (&characters, &states).iter() {
                        camera.x = state.position[0];
                        camera.y = state.position[1];
                    }
                }

                let mut frame = graphics::Frame::new(&graphics, args.frame, &camera);

                // draw entities
                let states = planner.world.read::<physic::PhysicState>();
                let types = planner.world.read::<physic::PhysicType>();
                let colors = planner.world.read::<graphics::Color>();

                for (state, typ, color) in (&states, &types, &colors).iter() {
                    let x = state.position[0];
                    let y = state.position[1];
                    match typ.shape {
                        physic::Shape::Circle(radius) => frame.draw_circle(x,y,radius,graphics::Layer::Middle,*color),
                        physic::Shape::Square(radius) => frame.draw_square(x,y,radius,graphics::Layer::Middle,*color),
                    }
                }

                // draw grid
                // draw effects
            },
            Event::Input(InputEvent::Closed) => break,
            Event::Input(InputEvent::KeyboardInput(state,_keycode,_)) => {
                if state == ElementState::Pressed {
                    // app.key_pressed(keycode);
                } else {
                    // app.key_released(keycode);
                }
            },
            Event::Input(InputEvent::MouseInput(state,_button)) => {
                if state == ElementState::Pressed {
                    // app.mouse_pressed(button);
                } else {
                    // app.mouse_released(button);
                }
            },
            Event::Input(InputEvent::MouseMoved(_x,_y)) => {
                // app.mouse_moved(x,y);
            },
            Event::Input(_) => (),
            Event::Idle(args) => thread::sleep(Duration::from_millis(args.dt as u64)),
        }
    }
}

