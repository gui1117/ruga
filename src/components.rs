pub use ::weapons::components::*;
pub use ::physics::components::*;

pub fn register_components(world: &mut ::specs::World) {
    ::weapons::components::register_components(world);
    ::physics::components::register_components(world);
    world.register::<PlayerControl>();
}

#[derive(Clone, Default)]
pub struct PlayerControl;
impl ::specs::Component for PlayerControl {
    type Storage = ::specs::NullStorage<Self>;
}
