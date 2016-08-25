use specs;
use graphics;

#[derive(Clone,Default)]
pub struct FixedCamera;
impl specs::Component for FixedCamera {
    type Storage = specs::NullStorage<Self>;
}

pub struct FixedCameraText {
    pub string: String,
}

impl specs::Component for FixedCameraText {
    type Storage = specs::VecStorage<Self>;
}

impl FixedCameraText {
    pub fn new(s: String) -> Self {
        FixedCameraText {
            string: s,
        }
    }
}

pub struct Text {
    pub lines: Vec<graphics::Line>,
    pub string: String,
}
impl specs::Component for Text {
    type Storage = specs::VecStorage<Self>;
}

impl Text {
    pub fn new(text: String, lines: Vec<graphics::Line>) -> Self {
        Text {
            lines: lines,
            string: text,
        }
    }
}
