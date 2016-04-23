extern crate baal;
extern crate thigra;
extern crate glium;
extern crate hlua;
extern crate specs;
extern crate yaml_unifier;
extern crate yaml_rust;
extern crate time;

pub mod components;
mod levels;
mod event_loop;
mod window;
#[macro_use]
pub mod entity_macro;
pub mod entities;

use event_loop::{ Events, Event};
use yaml_rust::yaml;

#[derive(Clone)]
struct Position {
    truc: f32,
}

impl specs::Component for Position {
    type Storage = specs::VecStorage<Self>;
}

fn afficher(p: &Position) {
    println!("{}",p.truc);
}

fn main() {
    let config = yaml_unifier::unify(std::path::Path::new("config")).unwrap();

    // init baal
    let audio_config = config.get(&yaml::Yaml::String("audio".into())).unwrap();
    let baal_setting = baal::Setting::from_yaml(audio_config).unwrap();
    baal::init(&baal_setting).unwrap();

    // init window
    let window_config = config.get(&yaml::Yaml::String("window".into())).unwrap();
    let window_setting = window::Setting::from_yaml(window_config).unwrap();
    let window = window::create(&window_setting).unwrap();

    // init thigra
    let graphics_config = config.get(&yaml::Yaml::String("graphics".into())).unwrap();
    let thigra_setting = thigra::Setting::from_yaml(graphics_config).unwrap();
    let t_graphics = thigra::TGraphics::new(&window, thigra_setting).unwrap();

    // init entities
    let entities_config = config.get(&yaml::Yaml::String("entities".into())).unwrap();
    let entities_setting = entities::Setting::from_yaml(entities_config).unwrap();

    // init levels

    // init world
    let mut world = specs::World::new();

    // fill world

    // init event loop
    let event_loop_config = config.get(&yaml::Yaml::String("event_loop".into())).unwrap();
    let event_loop_setting = event_loop::Setting::from_yaml(event_loop_config).unwrap();
    let mut window_events = window.events(&event_loop_setting);

    // game loop
    // while let Some(event) = window_events.next(&mut window) {
    //     match event {
    //         Event::Render(args) => app.render(args),
    //         Event::Update(args) => app.update(args),
    //         Event::Input(InputEvent::Closed) => break,
    //         Event::Input(InputEvent::KeyboardInput(state,keycode,_)) => {
    //             if state == ElementState::Pressed {
    //                 app.key_pressed(keycode);
    //             } else {
    //                 app.key_released(keycode);
    //             }
    //         },
    //         Event::Input(InputEvent::MouseInput(state,button)) => {
    //             if state == ElementState::Pressed {
    //                 app.mouse_pressed(button);
    //             } else {
    //                 app.mouse_released(button);
    //             }
    //         },
    //         Event::Input(InputEvent::MouseMoved((x,y))) => {
    //             app.mouse_moved(x,y);
    //         },
    //         Event::Input(_) => (),
    //         Event::Idle(args) => thread::sleep(Duration::from_millis(args.dt as u64)),
    //     }
    //     if app.quit { break; }
    // }

    // test
    // let p = Position { truc: 0f32 };

    // world.register::<Position>();
    // world.create_now()
    //     .build();
    // let mut planner = specs::Planner::new(world,4);
    // planner.run0w1r(afficher);
}

