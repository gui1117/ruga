use yaml_rust::yaml;
use yaml_utils;

use control::PlayerControl;
use physic::{
    PhysicState,
    PhysicType,
    PhysicForce,
    Shape,
    CollisionBehavior,
};
use graphics::Color;

// yaml!(
// pub struct EntitiesSetting {
//     pub character: CharacterSetting,
//     pub monster: MonsterSetting,
// });

// yaml!(
// pub struct CharacterSetting {
// });

// yaml!(
// pub struct MonsterSetting {
// });

pub struct Entities {
    pub character: Character,
    pub monster: Monster,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            character: Character::new(),
            monster: Monster::new(),
        }
    }
}

pub struct Character {
    physic_state: PhysicState,
    physic_type: PhysicType,
    physic_force: PhysicForce,
    color: Color,
    control: PlayerControl,
}

impl Character {
    pub fn new() -> Self {
        Character {
            physic_state: PhysicState::new(),
            physic_type: PhysicType::new(
                Shape::Circle(1.),
                CollisionBehavior::Persist,
                1.,
                0.1,
                1.),

            physic_force: PhysicForce::new(),
            color: Color::Yellow,
            control: PlayerControl,
        }
    }
}

pub struct Monster {
    physic_state: PhysicState,
    physic_type: PhysicType,
    physic_force: PhysicForce,
    color: Color,
}

impl Monster {
    pub fn new() -> Self {
        Monster {
            physic_state: PhysicState::new(),
            physic_type: PhysicType::new(
                Shape::Circle(1.),
                CollisionBehavior::Persist,
                1.,
                0.1,
                1.),

            physic_force: PhysicForce::new(),
            color: Color::Yellow,
        }
    }
}


// entity!(Character,"character" =>
//         physic_type,"physic_type" => PhysicType,PhysicType,
//         physic_force,"physic_force" => PhysicForce,PhysicForce,
//         physic_state,"physic_state" => PhysicState,PhysicState,
//         color, "color" => Color,Color);

