use specs;
use graphics::Layer;
use components::*;
use physics::Shape;

#[cfg_attr(rustfmt, rustfmt_skip)]const WALL_GROUP:       u32 = 0b00000000000000000000000000000001;
#[cfg_attr(rustfmt, rustfmt_skip)]const CHARACTER_GROUP:  u32 = 0b00000000000000000000000000000010;

#[cfg_attr(rustfmt, rustfmt_skip)]const WALL_MASK:        u32 = 0b11111111111111111111111111111111;
#[cfg_attr(rustfmt, rustfmt_skip)]const CHARACTER_MASK:   u32 = 0b11111111111111111111111111111111;

macro_rules! impl_entity {
    ($($entity:ident($($var_name:ident: $var_type:ident),*),)*) => {
        macro_rules! entities_signatures {
            () => {
                fn add_wall(x: f32, y: f32, width: f32, height: f32);
            }
        }
    }
}

impl_entity! {
    wall(x: f32, y: f32, width: f32, height: f32),
}

pub fn add_wall(world: &mut specs::World, x: f32, y: f32, width: f32, height: f32) {
    let shape = Shape::Rectangle(width, height);
    let entity = world.create_now()
        .with(PhysicState::new([x, y]))
        .with(PhysicType::new_static(WALL_GROUP, WALL_MASK, shape))
        .with(PhysicStatic)
        .build();
}

// pub fn add_character(world: &mut specs::World, x: f32, y: f32, r: f32) {
//     let shape = Shape::Circle(r);
//     let entity = world.create_now()
//         .with(PhysicState::new([x, y]))
//         .with(PhysicType::new_movable(CHARACTER_GROUP,
//                                       CHARACTER_MASK,
//                                       shape,
//                                       CollisionBehavior::Persist,
//                                       ,
//                                       CHARACTER_TIME_TO_REACH_VMAX,
//                                       CHARACTER_WEIGHT))
//         .with(PhysicDynamic)
//         .with(PlayerControl)
//         .build();
// }
