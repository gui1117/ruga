use specs;

#[derive(Clone,Default)]
pub struct FixedCamera;
impl specs::Component for FixedCamera {
    type Storage = specs::NullStorage<Self>;
}

pub struct Text {
    pub string: String,
}

impl specs::Component for Text {
    type Storage = specs::VecStorage<Self>;
}

impl Text {
    pub fn new(s: String) -> Self {
        Text {
            string: s,
        }
    }
}
