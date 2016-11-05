use graphics;
use specs;

#[derive(Clone,Default)]
pub struct HitboxIdFlag;
impl specs::Component for HitboxIdFlag {
    type Storage = specs::NullStorage<Self>;
}

#[derive(Clone)]
pub struct HitboxDraw {
    pub color: [f32;4],
    pub layer: graphics::Layer,
}
impl HitboxDraw {
    pub fn new(color: [f32;4], layer: graphics::Layer) -> HitboxDraw {
        HitboxDraw {
            color: color,
            layer: layer,
        }
    }
}
impl specs::Component for HitboxDraw {
    type Storage = specs::VecStorage<Self>;
}

#[derive(Clone,Default)]
pub struct PlayerControl;
impl specs::Component for PlayerControl {
    type Storage = specs::NullStorage<Self>;
}

#[derive(Clone,Copy)]
pub enum CollisionBehavior {
    Dodge,
    // TODO Bounce,
    Back,
}
impl specs::Component for CollisionBehavior {
    type Storage = specs::VecStorage<Self>;
}
