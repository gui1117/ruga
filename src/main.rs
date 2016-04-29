extern crate baal;
extern crate graphics;
extern crate glium;
extern crate hlua;
extern crate specs;
#[macro_use] extern crate yaml_utils;
extern crate yaml_rust;
extern crate time;

// mod levels;
mod event_loop;
mod window;
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
    let graphics_setting = graphics::Setting::from_yaml(&graphics_config).unwrap();
    let t_graphics = graphics::TGraphics::new(&window, graphics_setting).unwrap();

    // init entities
    let entities_config = config.remove(&yaml::Yaml::String("entities".into())).unwrap();
    let entities_setting = entities::Setting::from_yaml(entities_config).unwrap();
    let entities = entities::Entities::new(entities_setting);

    // init world
    let mut world = specs::World::new();
    world.register::<physic::PhysicState>();
    world.register::<physic::PhysicType>();
    world.register::<physic::PhysicForce>();
    world.register::<graphics::Color>();

    // load level
    // levels::load("toto".into(),&mut world,&entities_setting);

    // init event loop
    let event_loop_config = config.get(&yaml::Yaml::String("event_loop".into())).unwrap();
    let event_loop_setting = event_loop::Setting::from_yaml(event_loop_config).unwrap();
    let mut window_events = window.events(&event_loop_setting);

    // game loop
    while let Some(event) = window_events.next(&mut window) {
        match event {
            Event::Render(_args) => {
            },
            Event::Update(_args) => {
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

    // test
    // let p = Position { truc: 0f32 };

    // world.register::<Position>();
    // world.create_now()
    //     .build();
    // let mut planner = specs::Planner::new(world,4);
    // planner.run0w1r(afficher);
}

