use components::*;
use specs;
use config;

pub fn add_character(world: &specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicDynamic>(PhysicDynamic)
        .with::<PhysicType>(PhysicType::new_movable(
                config.entities.char_group.val,
                config.entities.char_mask.val,
                Shape::Circle(config.entities.char_radius),
                CollisionBehavior::Persist,
                config.entities.char_velocity,
                config.entities.char_time,
                config.entities.char_weight))
        .with::<PhysicForce>(PhysicForce::new())
        .with::<Life>(Life::new())
        .with::<Color>(Color::from_str(&*config.entities.char_color))
        .with::<PlayerControl>(PlayerControl)
        .build();
    world.create_now()
        .with::<GridSquare>(GridSquare::new(pos))
        .with::<Color>(Color::from_str(&*config.entities.portal_end_color))
        .build();
}

pub fn add_wall(world: &specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicStatic>(PhysicStatic)
        .with::<PhysicType>(PhysicType::new_static(
                config.entities.wall_group.val,
                config.entities.wall_mask.val,
                Shape::Square(config.entities.wall_radius)))
        .with::<Color>(Color::from_str(&*config.entities.wall_color))
        .build();
}

pub fn add_column(world: &specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicStatic>(PhysicStatic)
        .with::<PhysicType>(PhysicType::new_static(
                config.entities.column_group.val,
                config.entities.column_mask.val,
                Shape::Square(config.entities.column_radius)))
        .with::<Color>(Color::from_str(&*config.entities.column_color))
        .build();
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<Ball>(Ball::new(pos))
        .with::<PhysicDynamic>(PhysicDynamic)
        .with::<PhysicType>(PhysicType::new_movable(
                config.entities.ball_group.val,
                config.entities.ball_mask.val,
                Shape::Circle(config.entities.ball_radius),
                CollisionBehavior::Persist,
                config.entities.ball_velocity,
                config.entities.ball_time,
                config.entities.ball_weight))
        .with::<PhysicForce>(PhysicForce::new_full())
        .with::<PhysicTrigger>(PhysicTrigger::new())
        .with::<Life>(Life::new())
        .with::<Color>(Color::from_str(&*config.entities.ball_color))
        .with::<TowardPlayerControl>(TowardPlayerControl)
        .with::<Killer>(Killer {
            kamikaze: false,
            mask: config.entities.ball_killer_mask.val,
        })
        .build();
}

pub fn add_monster(world: &specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicDynamic>(PhysicDynamic)
        .with::<PhysicType>(PhysicType::new_movable(
                config.entities.monster_group.val,
                config.entities.monster_mask.val,
                Shape::Circle(config.entities.monster_radius),
                CollisionBehavior::Persist,
                config.entities.monster_velocity,
                config.entities.monster_time,
                config.entities.monster_weight))
        .with::<PhysicForce>(PhysicForce::new())
        .with::<Life>(Life::new())
        .with::<Color>(Color::from_str(&*config.entities.monster_color))
        .with::<MonsterControl>(MonsterControl::new())
        .with::<Killer>(Killer {
            kamikaze: true,
            mask: config.entities.monster_killer_mask.val,
        })
        .build();
}

pub fn add_laser(world: &specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicStatic>(PhysicStatic)
        .with::<PhysicType>(PhysicType::new_static(
                config.entities.laser_group.val,
                config.entities.laser_mask.val,
                Shape::Square(config.entities.laser_radius)))
        .with::<Color>(Color::from_str(&*config.entities.laser_color))
        .with::<Killer>(Killer {
            kamikaze: false,
            mask: config.entities.laser_killer_mask.val,
        })
        .build();
}

pub fn add_portal(world: &specs::World, pos: [isize;2], destination: String) {
    world.create_now()
        .with::<Portal>(Portal::new(destination))
        .with::<GridSquare>(GridSquare::new(pos))
        .with::<Color>(Color::from_str(&*config.entities.portal_start_color))
        .build();
}

