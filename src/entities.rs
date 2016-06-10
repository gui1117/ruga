use control::PlayerControl;
use weapons::{
    Rifle,
};
use physic::{
    PhysicState,
    PhysicType,
    PhysicForce,
    PhysicDynamic,
    Shape,
    CollisionBehavior,
};
use graphics::Color;
use specs;

fn add_character(world: &specs::World, pos: [f32;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicDynamic>(PhysicDynamic)
        .with::<PhysicType>(PhysicType::new_movable(
                Shape::Circle(1.),
                CollisionBehavior::Persist,
                30.,
                0.2,
                1.))
        .with::<PhysicForce>(PhysicForce::new())
        .with::<Color>(Color::Yellow)
        .with::<PlayerControl>(PlayerControl)
        // .with::<Rifle>(Rifle {
        //     rate: 1.,
        //     length: 20.,
        //     damage: 1.,
        //     shoot: false,
        //     recovery: 0.,
        //     ammo: 64,
        //     aim: 0.,
        // })
        .build();
}

fn add_wall(world: &specs::World, pos: [isize;2], radius: usize) {
    if radius == 0 { return; }

    unimplemented!();
}

// fn add_sensor_zone(world: &specs::World, pos: [isize;2], radius: usize) -> specs::Entity {
// }

// fn add_door(world: &specs::World, pos: [isize;2], radius: usize, signal: specs::Entity) {
// }

// fn add_signal_multiplexer(world: &specs::World, signals: Vec<specs::Entity>, code: String) -> specs::Entity {
// }

// fn add_door(world: &specs::World, pos: [isize;2], radius: usize, lock) {
// }


