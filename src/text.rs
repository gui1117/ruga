use specs;

#[derive(Clone, Default)]
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
    pub string: String,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
}
impl specs::Component for Text {
    type Storage = specs::VecStorage<Self>;
}

impl Text {
    pub fn new(x: f32, y: f32, scale: f32, text: String) -> Self {
        Text {
            string: text,
            scale: scale,
            x: x,
            y: y,
        }
    }
}
