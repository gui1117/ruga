#[macro_use]
extern crate glium;

extern crate rand;
extern crate time;
extern crate sndfile;
extern crate portaudio;


#[macro_use]
pub mod util;

pub mod world;
pub mod app;
pub mod maze;
pub mod sound_manager;
pub mod frame_manager;
pub mod effect_manager;

mod event_loop;

use app::App;
use glium::DisplayBuild;
use glium::glutin::Event as InputEvent;
use glium::glutin::ElementState;
use event_loop::{
    Events,
    Event,
};

fn main() {
    let mut window = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();

    let mut app = App::new(&window);

    let mut window_events = window.events();
    while let Some(event) = window_events.next(&mut window) {
        match event {
            Event::Render(args) => app.render(args),
            Event::Update(args) => app.update(args),
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
            Event::Input(InputEvent::MouseMoved((x,y))) => {
                app.mouse_moved(x,y);
            },
            Event::Input(_) => (),
            Event::Idle(_) => (),
        }
        if app.quit { break; }
    }
}
