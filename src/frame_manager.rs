use glium::Frame;

pub struct Assets;

impl Assets {
    pub fn new() -> Assets {
        Assets
    }
}

pub struct FrameManager<'l> {
    frame: Frame,
    ext_dt: f64,
    x: f64,
    y: f64,
    zoom: f64,
    assets: &'l Assets
}

impl<'l> FrameManager<'l> {
    pub fn new(assets: &'l Assets, frame: Frame, ext_dt: f64, x: f64, y: f64, zoom: f64) -> FrameManager<'l> {
        FrameManager {
            frame: frame,
            ext_dt: ext_dt,
            x: x,
            y: y,
            zoom: zoom,
            assets: assets,
        }
    }

    pub fn finish(self) {
        self.frame.finish().unwrap();
    }
}
