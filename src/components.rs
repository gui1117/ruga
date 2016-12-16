use graphics;
use physics::{CollisionBehavior, Shape, PHYSIC_RATE};
use specs;
use weapon;

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

impl_component! {
    PhysicState: VecStorage,
    PhysicType: VecStorage,
    PhysicDamping: VecStorage,
    PhysicForce: VecStorage,
    PhysicDynamic: NullStorage,
    PhysicStatic: NullStorage,
    DrawPhysic: VecStorage,
    PlayerControl: NullStorage,
    Aim: HashMapStorage,
    Shoot: NullStorage,
    Weapon: HashMapStorage,
    NextWeapon: HashMapStorage,
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
    pub weight: f32,
    pub group: u32,
    pub mask: u32,
}
impl PhysicType {
    pub fn new_movable(group: u32, mask: u32, shape: Shape, collision: CollisionBehavior, weight: f32) -> PhysicType {
        PhysicType {
            shape: shape,
            collision: collision,
            weight: weight,
            group: group,
            mask: mask,
        }
    }
    pub fn new_static(group: u32, mask: u32, shape: Shape) -> PhysicType {
        PhysicType {
            shape: shape,
            collision: CollisionBehavior::Persist,
            weight: ::std::f32::MAX,
            group: group,
            mask: mask,
        }
    }
}

#[derive(Clone)]
pub struct PhysicForce {
    pub angle: f32,
    pub strength: f32,
    pub coef: f32,
}

#[derive(Clone)]
pub struct PhysicDamping(pub f32);

#[derive(Clone)]
pub struct DrawPhysic {
    pub border: Option<(f32, [f32;4])>,
    pub color: [f32; 4],
}

#[derive(Clone)]
pub struct Aim(pub f32);

#[derive(Clone,Copy,Default)]
pub struct Shoot;

#[derive(Clone)]
pub struct Weapon {
    pub reload_factor: f32,
    pub setup_factor: f32,
    pub setdown_factor: f32,
    pub state: weapon::State,
    pub kind: weapon::Kind,
}

#[derive(Clone)]
pub struct NextWeapon(pub Weapon);

#[derive(Clone,Copy,Default)]
pub struct PhysicDynamic;
#[derive(Clone,Copy,Default)]
pub struct PhysicStatic;

#[derive(Clone,Copy,Default)]
pub struct PlayerControl;
