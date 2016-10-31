use api;

pub struct App {
    must_quit: bool
}

impl App {
    pub fn new() -> Self {
        App {
            must_quit: false,
        }
    }
    pub fn update(&mut self, _dt: f32) {
    }
    pub fn draw(&mut self) {
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
