use components::*;
use specs;
use config;
use levels;
use std::sync::Arc;

pub fn add_character(world: &mut specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicDynamic>(PhysicDynamic)
        .with::<PhysicType>(PhysicType::new_movable(
                config.entities.char_group,
                config.entities.char_mask,
                Shape::Circle(config.entities.char_radius),
                CollisionBehavior::Persist,
                config.entities.char_velocity,
                config.entities.char_time,
                config.entities.char_weight))
        .with::<PhysicForce>(PhysicForce::new())
        .with::<Life>(Life::new(config.entities.char_die_snd))
        .with::<Graphic>(Graphic::new(
                config.entities.char_color,
                config.entities.char_layer))
        .with::<PlayerControl>(PlayerControl)
        .build();
    world.create_now()
        .with::<GridSquare>(GridSquare::new(pos))
        .with::<Graphic>(Graphic::new(
                config.entities.portal_end_color,
                config.entities.portal_end_layer))
        .build();
}

pub fn add_wall(world: &mut specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicStatic>(PhysicStatic)
        .with::<PhysicType>(PhysicType::new_static(
                config.entities.wall_group,
                config.entities.wall_mask,
                Shape::Square(config.entities.wall_radius)))
        .with::<Graphic>(Graphic::new(
                config.entities.wall_color,
                config.entities.wall_layer))
        .build();
}

pub fn add_column(world: &mut specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<Column>(Column::new(config.entities.column_spawn_snd))
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicStatic>(PhysicStatic)
        .with::<PhysicType>(PhysicType::new_static(
                config.entities.column_group,
                config.entities.column_mask,
                Shape::Square(config.entities.column_radius)))
        .with::<Graphic>(Graphic::new(
                config.entities.column_color,
                config.entities.column_layer))
        .build();
}

pub fn add_ball(world: &mut specs::World, pos: [f32;2], arc: Arc<()>) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<Ball>(Ball::new(arc))
        .with::<PhysicDynamic>(PhysicDynamic)
        .with::<PhysicType>(PhysicType::new_movable(
                config.entities.ball_group,
                config.entities.ball_mask,
                Shape::Circle(config.entities.ball_radius),
                CollisionBehavior::Persist,
                config.entities.ball_velocity,
                config.entities.ball_time,
                config.entities.ball_weight))
        .with::<PhysicForce>(PhysicForce::new_full())
        .with::<PhysicTrigger>(PhysicTrigger::new())
        .with::<Life>(Life::new(config.entities.ball_die_snd))
        .with::<Graphic>(Graphic::new(
                config.entities.ball_color,
                config.entities.ball_layer))
        .with::<TowardPlayerControl>(TowardPlayerControl)
        .with::<Killer>(Killer {
            kamikaze: false,
            mask: config.entities.ball_killer_mask,
            kill_snd: config.entities.ball_kill_snd,
        })
        .build();
}

pub fn add_monster(world: &mut specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicDynamic>(PhysicDynamic)
        .with::<PhysicType>(PhysicType::new_movable(
                config.entities.monster_group,
                config.entities.monster_mask,
                Shape::Circle(config.entities.monster_radius),
                CollisionBehavior::Persist,
                config.entities.monster_velocity,
                config.entities.monster_time,
                config.entities.monster_weight))
        .with::<PhysicForce>(PhysicForce::new())
        .with::<Life>(Life::new(config.entities.monster_die_snd))
        .with::<Graphic>(Graphic::new(
                config.entities.monster_color,
                config.entities.monster_layer))
        .with::<MonsterControl>(MonsterControl::new())
        .with::<Killer>(Killer {
            kamikaze: true,
            mask: config.entities.monster_killer_mask,
            kill_snd: config.entities.monster_kill_snd,
        })
        // .with::<DynPersistentSnd>(DynPersistentSnd::new(
        //         config.entities.monster_persistent_snd))
        .build();
}

pub fn add_laser(world: &mut specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicStatic>(PhysicStatic)
        .with::<PhysicType>(PhysicType::new_static(
                config.entities.laser_group,
                config.entities.laser_mask,
                Shape::Square(config.entities.laser_radius)))
        .with::<Graphic>(Graphic::new(
                config.entities.laser_color,
                config.entities.laser_layer))
        .with::<Killer>(Killer {
            kamikaze: false,
            mask: config.entities.laser_killer_mask,
            kill_snd: config.entities.laser_kill_snd,
        })
        // .with::<StaticPersistentSnd>(StaticPersistentSnd::new(
        //         config.entities.laser_persistent_snd))
        .build();
}

pub fn add_portal(world: &mut specs::World, pos: [isize;2], destination: levels::Level) {
    world.create_now()
        .with::<Portal>(Portal::new(destination))
        .with::<GridSquare>(GridSquare::new(pos))
        .with::<Graphic>(Graphic::new(
                config.entities.portal_start_color,
                config.entities.portal_start_layer))
        .build();
}

pub fn add_fixed_camera_text(world: &mut specs::World, text: String) {
    world.create_now()
        .with::<FixedCamera>(FixedCamera)
        .build();
    world.create_now()
        .with::<FixedCameraText>(FixedCameraText::new(text))
        .build();
}

pub fn add_text(world: &mut specs::World, x: f32, y: f32, scale: f32, text: String) {
    world.create_now()
        .with::<Text>(Text::new(x, y, scale, text))
        .build();
}
