#[macro_use]
extern crate glium;

extern crate rand;
extern crate sndfile;
extern crate portaudio;

extern crate time;

#[macro_use]
pub mod util;

pub mod world;
pub mod app;
pub mod maze;
pub mod sound_manager;
pub mod graphic_manager;

mod event_loop;

use app::App;
use glium::DisplayBuild;
use event_loop::{
    Events,
    Event,
};

fn main() {
    let mut window = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let mut app = App::new(640.,480.);
    
    let mut window_events = window.events();
    while let Some(event) = window_events.next(&mut window) {
        match event {
            Event::Render(args) => app.render(args),
            Event::Update(args) => app.update(args),
            Event::Input(input) => app.input(input),
            Event::Idle(_) => (),
        }
    }
}
