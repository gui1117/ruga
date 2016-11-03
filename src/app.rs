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
        let camera = Camera::new(10.0,20.0,0.05);
        let mut frame = Frame::new(&mut self.graphics, frame, &camera);
        frame.draw_circle(10.5, 20.5, 0.5, Layer::Middle, [0.0,0.0,0.5,0.5]);
        frame.draw_square(12.5, 22.5, 0.5, Layer::Middle, [0.5,0.0,0.0,0.5]);
        frame.draw_text(12.0, 22.0, 1.0, "Tgto", Layer::Middle, [0.0,0.5,0.0,0.5]);
        frame.draw_square(0.4, 0.4, 0.1, Layer::Billboard, [0.5,0.0,0.0,0.5]);
        frame.draw_text(0.3, 0.3, 0.2, "Toto", Layer::Billboard, [0.0,0.5,0.0,0.5]);
        frame.draw_line((10.1,25.1), (10.1,25.3), (10.1,28.3), (15.3,28.3), 0.1, Layer::Middle, [0.0,0.5,0.5,0.5]);
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
