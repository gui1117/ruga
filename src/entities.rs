use yaml_rust::yaml;
use yaml_utils;

yaml!(
pub struct Setting {
    pub character: CharacterSetting,
});

yaml!(
pub struct CharacterSetting {
    // pub radius: f64,
    // pub max_velocity: f64,
    // pub time_to_reach_max_velocity: f64,
    // pub weight: f64,
});

pub struct Entities {
}

impl Entities {
    pub fn new(setting: Setting) -> Self {
        Entities {}
    }
}

// entity!(Character,"character" =>
//         physic_type,"physic_type" => PhysicType,PhysicType,
//         physic_force,"physic_force" => PhysicForce,PhysicForce,
//         physic_state,"physic_state" => PhysicState,PhysicState,
//         color, "color" => Color,Color);

