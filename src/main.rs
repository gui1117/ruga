extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate viewport;
extern crate rand;
extern crate sndfile;
extern crate portaudio;

#[macro_use]
pub mod util;
pub mod world;
pub mod app;
pub mod maze;
pub mod sound_manager;

use piston::window::{ WindowSettings, AdvancedWindow };
use piston::event_loop::Events;
use piston::input::{
    RenderEvent, 
    UpdateEvent,
    Event,
    Input,
};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use app::App;

fn main() {
    let opengl = OpenGL::V3_2;
    let window: Window = WindowSettings::new("ruga", [640, 480])
        .opengl(opengl)
        .exit_on_esc(false)
        .build()
        .unwrap();

    let gl = opengl_graphics::GlGraphics::new(opengl);
    let mut app = App::new(gl,640.,480.);

    let mut event_loop = window.events();
    loop {
        if app.quit { return; } 

        match event_loop.next().unwrap() {
            Event::Render(args) => app.render(&args),
            Event::Update(args) => app.update(&args),
            Event::AfterRender(_args) => (),
            Event::Idle(_args) => (),
            Event::Input(Input::Press(button)) => app.press(&button),
            Event::Input(Input::Release(button)) => app.release(&button),
            Event::Input(Input::Move(motion)) => app.motion(&motion),
            Event::Input(Input::Text(_text)) => (),
            Event::Input(Input::Resize(_width, _height)) => (),
            Event::Input(Input::Focus(_focus)) => (),
            Event::Input(Input::Cursor(_cursor)) => (),
        }
    }
}
