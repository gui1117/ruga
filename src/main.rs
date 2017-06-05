#[macro_use] extern crate lazy_static;
extern crate toml;
extern crate graphics;
extern crate glium;
extern crate specs;
extern crate time;
extern crate rand;
extern crate fnv;
extern crate png;
extern crate gilrs;
#[macro_use] extern crate serde_derive;
extern crate serde;

mod levels;
mod app;
mod configuration;
mod event_loop;
mod control;
mod physic;
mod entities;
mod utils;
mod life;
mod portal;
mod text;
mod ui;

mod components {
    pub use control::{
        PlayerControl,
        TowardPlayerControl,
        MonsterControl,
    };
    pub use physic::{
        GridSquare,
        PhysicState,
        PhysicType,
        PhysicForce,
        PhysicDynamic,
        PhysicStatic,
        PhysicTrigger,
        Shape,
        Ray,
        CollisionBehavior,
    };
    pub use life::{
        Column,
        Life,
        Killer,
        Ball,
    };
    pub use portal::Portal;
    pub use app::Graphic;
    pub use text::{
        FixedCameraText,
        FixedCamera,
        Text,
    };
}
mod resource {
    pub use physic::PhysicWorld;
}
mod systems {
    pub use physic::PhysicSystem;
    pub use life::{
        LifeSystem,
        KillerSystem,
        BallSystem,
        ColumnSystem,
    };
    pub use control::{
        PlayerSystem,
        MonsterSystem,
        TowardPlayerSystem,
    };
    pub use portal::PortalSystem;
}

pub use configuration::CONFIG as config;

use glium::glutin;
use glium::DisplayBuild;
use event_loop::{
    Events,
    Event,
};
use utils::OkOrExit;

use std::thread;

fn main() {
    safe_main().ok_or_exit();
}

fn safe_main() -> Result<(), String> {
    // load casltes
    let castles = levels::load_castles()
        .map_err(|e| format!("load castle: {}", e))?;

    // init window
    // TODO if fail then disable vsync and then multisampling and then vsync and multisamping
    let mut window = {
        let mut builder = glium::glutin::WindowBuilder::new()
            .with_title(format!("ruga"));

        if config.window.vsync {
            builder = builder.with_vsync();
        }
        if config.window.multisampling != 0 {
            builder = builder.with_multisampling(config.window.multisampling);
        }
        if config.window.fullscreen {
            if config.window.fullscreen_on_primary_monitor {
                builder = builder.with_fullscreen(glutin::get_primary_monitor());
            } else {
                builder = builder.with_fullscreen(glutin::get_available_monitors().nth(config.window.fullscreen_monitor)
                                                  .ok_or("fullsceen monitor specified unavailable")?);
            }
        } else {
            builder = builder.with_dimensions(config.window.dimension[0], config.window.dimension[1])
        }
        try!(builder.build_glium().map_err(|e| format!("window init: {}", e)))
    };
    window.get_window().unwrap().set_cursor_state(glium::glutin::CursorState::Hide).unwrap();

    // init app
    let mut app = try!(app::App::new(&window, castles).map_err(|e| format!("app creation: {}", e)));

    // init event loop
    let mut window_events = window.events(&event_loop::Setting {
        ups: config.event_loop.ups,
        max_fps: config.event_loop.max_fps,
    });

    let mut gamepad = gilrs::Gilrs::new();

    // game loop
    while let Some(event) = window_events.next(&mut window, &mut gamepad) {
        match event {
            Event::Update(args) => app.update(args),
            Event::Render(args) => app.render(args),
            Event::GlutinEvent(glutin::Event::Closed) => break,
            Event::GlutinEvent(glutin::Event::KeyboardInput(state, keycode, _)) => {
                if state == glutin::ElementState::Pressed {
                    app.key_pressed(keycode);
                } else {
                    app.key_released(keycode);
                }
            },
            Event::GlutinEvent(glutin::Event::Focused(f)) => app.focused(f),
            Event::GlutinEvent(glutin::Event::Touch(t)) => app.touch(t),
            Event::GlutinEvent(_) => (),
            Event::GilrsEvent(gilrs::Event::ButtonPressed(button, _)) => app.button_pressed(button),
            Event::GilrsEvent(gilrs::Event::ButtonReleased(button, _)) => app.button_released(button),
            Event::GilrsEvent(gilrs::Event::AxisChanged(axis, pos, _)) => app.axis_changed(axis, pos),
            Event::GilrsEvent(_) => (),
            Event::Idle(args) => thread::sleep(args.dt),
        }

        if app.quit {
            break;
        }
    }

    return Ok(());
}
