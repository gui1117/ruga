use specs;
use yaml_rust::yaml;
use components::*;

#[derive(Debug,Clone)]
pub struct Setting {
    pub character: Character,
}

impl Setting {
    pub fn from_yaml(config: &yaml::Yaml) -> Result<Setting,String> {
        let hash = try!(config.as_hash().ok_or("entities setting must be associative array"));

        let character = try!(Character::from_yaml(try!(hash.get(&yaml::Yaml::from_str("character"))
                                                       .ok_or("entities setting must have character key"))));

        Ok(Setting {
            character: character,
        })
    }
}

entity!(Character,"character" =>
        physic_type,"physic_type" => PhysicType,PhysicType,
        physic_force,"physic_force" => PhysicForce,PhysicForce,
        physic_state,"physic_state" => PhysicState,PhysicState,
        color, "color" => Color,Color);
