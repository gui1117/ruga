pub use ::physics::resources::*;
pub use ::notifications::resources::*;

pub fn add_resources(world: &mut ::specs::World) {
    ::physics::resources::add_resources(world);
    ::notifications::resources::add_resources(world);
    world.add_resource(Notifications::new());
    world.add_resource(Cursor::new());
    world.add_resource(Zoom::new());
}

pub struct Zoom(pub f32);
impl Zoom {
    pub fn new() -> Self {
        Zoom(0.05)
    }
}

pub struct Cursor {
    pub x: f32,
    pub y: f32,
}
impl Cursor {
    pub fn new() -> Self {
        Cursor { x: 0., y: 0. }
    }
}
