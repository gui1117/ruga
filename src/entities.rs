use specs;
use weapon;
use graphics::Layer;
use specs::InsertResult;
use components::*;
use physics::{self, Shape, CollisionBehavior};
use std::f32;

macro_rules! entity_builder {
    ($($entity:ident($($var_name:ident: $var_type:ident),*),)*) => {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        pub enum EntityBuilderMsg {
            $($entity((), $($var_type),*)),*
        }
        pub trait EntityBuilder {
            $(fn $entity(&mut self, $($var_name: $var_type),*);)*
            fn build_entity(&mut self, msg: EntityBuilderMsg) {
                match msg {
                    $( EntityBuilderMsg::$entity(_, $($var_name),*) =>
                       self.$entity($($var_name),*),)*
                }
            }
        }
        macro_rules! impl_entity_builder {
            ($ty:ty) => {
                impl ::entities::EntityBuilder for $ty {
                    $( fn $entity(&mut self, $($var_name: $var_type),*) {
                        let world = self.planner.mut_world();
                        ::entities::$entity(world, $($var_name),*);
                    } )*
                }
            }
        }
        pub fn set_lua_builder(lua: &mut ::hlua::Lua,
                               sender: ::std::sync::mpsc::Sender<::api::CallerMsg>) {
            use ::api::CallerMsg::EntityBuilder;
            $(
                let sender_clone = sender.clone();
                let func = stringify!($entity);
                lua.set(func, infer_type!($($var_name)*)(move |$($var_name),*| {
                    sender_clone.send(
                        EntityBuilder(EntityBuilderMsg::$entity((), $($var_name),*))
                        ).unwrap();
                }));
            )*
        }
        pub fn builder_function_names() -> Vec<String> {
            vec!($( String::from(stringify!($entity))),*)
        }
    }
}

// TODO add documentation ?
entity_builder! {
    add_wall(x: f32, y: f32, width: f32, height: f32),
    add_character(x: f32, y: f32, velocity: f32, time_to_reach_v_max: f32, weight: f32),
}

const WALL_GROUP:  u32 = 0b00000000000000000000000000000001;
const CHAR_GROUP:  u32 = 0b00000000000000000000000000000010;

const WALL_MASK:   u32 = 0b11111111111111111111111111111111;
const CHAR_MASK:   u32 = 0b11111111111111111111111111111111;

pub fn add_wall(world: &mut specs::World, x: f32, y: f32, width: f32, height: f32) {
    let shape = Shape::Rectangle(width, height);
    world.create_now()
        .with(PhysicState::new([x, y]))
        .with(PhysicType::new_static(WALL_GROUP, WALL_MASK, shape))
        .with(PhysicStatic)
        .with(DrawPhysic {
            color: [0., 0., 0., 1.],
            border: None,
        })
        .build();
}

pub fn add_character(world: &mut specs::World, x: f32, y: f32, velocity: f32, time_to_reach_vmax: f32, weight: f32) {
    let shape = Shape::Circle(0.5);
    let (force, damping) = physics::compute_force_damping(velocity, time_to_reach_vmax, weight);
    let char_entity = world.create_now()
        .with(PhysicState::new([x, y]))
        .with(PhysicType::new_movable(CHAR_GROUP, CHAR_MASK, shape, CollisionBehavior::Persist, weight))
        .with(PhysicForce {
            angle: 0.,
            strength: 0.,
            coef: force,
        })
        .with(PhysicDamping(damping))
        .with(PhysicDynamic)
        .with(PlayerControl)
        .with(Aim(0.5))
        .with(DrawPhysic {
            color: [1., 1., 1., 1.],
            border: Some((0.15, [0., 0., 0., 1.])),
        })
        .build();
}
