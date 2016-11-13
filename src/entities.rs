use specs;
use graphics::Layer;
use components::*;
use physics::Shape;

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
                    fn add_wall(&mut self, x: f32, y: f32, width: f32, height: f32) {
                        let world = self.planner.mut_world();
                        ::entities::add_wall(world, x, y, width, height);
                    }
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

entity_builder! {
    add_wall(x: f32, y: f32, width: f32, height: f32),
}

#[cfg_attr(rustfmt, rustfmt_skip)]const WALL_GROUP:       u32 = 0b00000000000000000000000000000001;
#[cfg_attr(rustfmt, rustfmt_skip)]const CHARACTER_GROUP:  u32 = 0b00000000000000000000000000000010;

#[cfg_attr(rustfmt, rustfmt_skip)]const WALL_MASK:        u32 = 0b11111111111111111111111111111111;
#[cfg_attr(rustfmt, rustfmt_skip)]const CHARACTER_MASK:   u32 = 0b11111111111111111111111111111111;

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
