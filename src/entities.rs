use control::PlayerControl;
use weapons::{
    Rifle,
    RifleState,
};
use physic::{
    PhysicState,
    PhysicType,
    PhysicForce,
    PhysicDynamic,
    PhysicStatic,
    Shape,
    CollisionBehavior,
};
use graphics::Color;
use specs;

pub fn add_character(world: &specs::World, pos: [f32;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicDynamic>(PhysicDynamic)
        .with::<PhysicType>(PhysicType::new_movable(
                Shape::Circle(0.5),
                CollisionBehavior::Persist,
                10.,
                0.05,
                1.))
        .with::<PhysicForce>(PhysicForce::new())
        .with::<Color>(Color::Yellow)
        .with::<PlayerControl>(PlayerControl)
        .with::<Rifle>(Rifle {
            length: 10.,
            max_ammo: 20.,
            ammo_regen: 1.,
            damage: 1.,
            rate: 0.1,
            lots: 5,
            deviation: 0.1,
            distance: 0.6,

            aim: 0.,
            ammo: 0.,
            state: RifleState::Rest,
            recovery: 0.,
        })
        .build();
}

pub fn add_wall(world: &specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new_aligned(pos))
        .with::<PhysicStatic>(PhysicStatic)
        .with::<PhysicType>(PhysicType::new_static(Shape::Square(0.5)))
        .with::<Color>(Color::Yellow)
        .build();
}

