use components::*;
use specs;
use config;

pub fn add_character(world: &specs::World, pos: [f32;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicDynamic>(PhysicDynamic)
        .with::<PhysicType>(PhysicType::new_movable(
                config.entity.char_group,
                Shape::Circle(config.entity.char_radius),
                CollisionBehavior::Persist,
                config.entity.char_velocity,
                config.entity.char_time,
                config.entity.char_weight))
        .with::<PhysicForce>(PhysicForce::new())
        .with::<Life>(Life::new())
        .with::<Color>(Color::from_str(&*config.entity.char_color))
        .with::<PlayerControl>(PlayerControl)
        .build();
}

pub fn add_wall(world: &specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicStatic>(PhysicStatic)
        .with::<PhysicType>(PhysicType::new_static(
                config.entity.wall_group,
                Shape::Square(config.entity.wall_radius)))
        .with::<Color>(Color::from_str(&*config.entity.wall_color))
        .build();
}

pub fn add_colunm(world: &specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicStatic>(PhysicStatic)
        .with::<PhysicType>(PhysicType::new_static(
                config.entity.column_group,
                Shape::Circle(config.entity.column_radius)))
        .with::<Color>(Color::from_str(&*config.entity.column_color))
        .build();
    //TODO create ball
}

pub fn add_monster(world: &specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicDynamic>(PhysicDynamic)
        .with::<PhysicType>(PhysicType::new_movable(
                config.entity.monster_group,
                Shape::Circle(config.entity.monster_radius),
                CollisionBehavior::Persist,
                config.entity.monster_velocity,
                config.entity.monster_time,
                config.entity.monster_weight))
        .with::<PhysicForce>(PhysicForce::new())
        .with::<Life>(Life::new())
        .with::<Color>(Color::from_str(&*config.entity.monster_color))
        .with::<MonsterControl>(MonsterControl::new())
        .build();
}
pub fn add_laser(world: &specs::World, pos: [isize;2]) {
}
// pub fn add_begin() {
// }
// pub fn add_end() {
// }
