use collider;
use graphics;
use specs;

pub struct HitboxIdT(pub collider::HitboxId);
impl specs::Component for HitboxIdT {
    type Storage = specs::VecStorage<Self>;
}

#[derive(Clone)]
pub struct HitboxDraw {
    pub color : [f32;4],
    pub layer : graphics::Layer,
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
    Bounce,
    Back,
}
impl specs::Component for CollisionBehavior {
    type Storage = specs::VecStorage<Self>;
}
