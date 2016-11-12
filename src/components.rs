use graphics;
use physics::{CollisionBehavior, Shape, PHYSIC_RATE};
use specs;

macro_rules! impl_component {
    ($($typ:ident: $storage:ident,)*) => {
        pub fn register_components(world: &mut ::specs::World) {
            $(world.register::<::components::$typ>();)*
        }

        $(impl ::specs::Component for $typ {
            type Storage = ::specs::$storage<Self>;
        })*
    };
}

impl_component!{
    PhysicState: VecStorage,
    PhysicType: VecStorage,
    PhysicForce: VecStorage,
    PhysicDynamic: NullStorage,
    PhysicStatic: NullStorage,
    DrawPhysic: VecStorage,
    PlayerControl: NullStorage,
}

#[derive(Clone)]
pub struct PhysicState {
    pub pos: [f32; 2],
    pub vel: [f32; 2],
    pub acc: [f32; 2],
}
impl PhysicState {
    pub fn new(pos: [f32; 2]) -> Self {
        PhysicState {
            pos: pos,
            vel: [0., 0.],
            acc: [0., 0.],
        }
    }
}

#[derive(Clone)]
pub struct PhysicType {
    pub shape: Shape,
    pub collision: CollisionBehavior,
    pub damping: f32,
    pub force: f32,
    pub weight: f32,
    pub group: u32,
    pub mask: u32,
}
impl PhysicType {
    pub fn new_movable(group: u32,
                       mask: u32,
                       shape: Shape,
                       collision: CollisionBehavior,
                       velocity: f32,
                       time_to_reach_v_max: f32,
                       weight: f32)
                       -> Self {
        let damping = -weight * (1. - PHYSIC_RATE).ln() / time_to_reach_v_max;
        let force = velocity * damping;
        PhysicType {
            shape: shape,
            collision: collision,
            weight: weight,
            damping: damping,
            force: force,
            group: group,
            mask: mask,
        }
    }
    pub fn new_static(group: u32, mask: u32, shape: Shape) -> Self {
        PhysicType {
            shape: shape,
            collision: CollisionBehavior::Persist,
            weight: ::std::f32::MAX,
            force: 0.,
            damping: 0.,
            group: group,
            mask: mask,
        }
    }
}

#[derive(Clone)]
pub struct PhysicForce {
    pub angle: f32,
    pub strength: f32,
}

#[derive(Clone)]
pub struct DrawPhysic {
    pub color: [f32; 4],
}

#[derive(Clone,Copy,Default)]
pub struct PhysicDynamic;
#[derive(Clone,Copy,Default)]
pub struct PhysicStatic;

#[derive(Clone,Copy,Default)]
pub struct PlayerControl;
