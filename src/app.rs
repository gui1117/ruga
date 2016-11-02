use api;
use glium;
use glium::backend::Facade;
use graphics::{Graphics, Frame, Camera, Layer};

pub struct App {
    must_quit: bool,
    graphics: Graphics,
}

impl App {
    pub fn new<F: Facade>(facade: &F) -> Self {
        App {
            graphics: Graphics::new(facade).unwrap(),
            must_quit: false,
        }
    }
    pub fn update(&mut self, _dt: f32) {
    }
    pub fn draw(&mut self, frame: glium::Frame) {
        let camera = Camera::new(20.0,100.0,0.1);
        let mut frame = Frame::new(&mut self.graphics, frame, &camera);
        frame.draw_circle(20.5, 100.5, 0.5, Layer::Middle, [0.0,0.0,0.5,0.5]);
        frame.draw_square(25.5, 105.5, 0.5, Layer::Middle, [0.5,0.0,0.0,0.5]);
        frame.draw_text(25.0, 105.0, 1.0, "Toto", Layer::Middle, [0.0,0.5,0.0,0.5]);
        // frame.draw_square(0.4, 0.4, 0.1, Layer::Billboard, [0.5,0.0,0.0,0.5]);
        // frame.draw_text(0.3, 0.3, 0.2, "Toto", Layer::Billboard, [0.0,0.5,0.0,0.5]);
        frame.finish().unwrap();
    }
    pub fn must_quit(&self) -> bool {
        self.must_quit
    }
}

impl api::Caller for App {
    fn quit(&mut self) {
        self.must_quit = true;
    }
    fn notify(&mut self, notification: String) {
        unimplemented!();
    }
}
