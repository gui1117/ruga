use specs;
use yaml_rust::yaml;
use thigra;

#[derive(Debug,Clone)]
pub enum Shape {
    Circle(f32),
    Square(f32),
}

#[derive(Debug,Clone)]
pub enum CollisionBehavior {
    Bounce,
    Persist,
    Stop,
}

#[derive(Debug,Clone)]
pub struct PhysicState {
    pub position: [f32;2],
    pub velocity: [f32;2],
    pub acceleration: [f32;2],
}
impl specs::Component for PhysicState {
    type Storage = specs::VecStorage<Self>;
}
impl PhysicState {
    pub fn from_yaml(config: &yaml::Yaml) -> Result<PhysicState,String> {
        if config.is_null() {
            Ok(PhysicState {
                position: [0.,0.],
                velocity: [0.,0.],
                acceleration: [0.,0.],
            })
        } else {
            Err("".into())
        }
    }
}

#[derive(Debug,Clone)]
pub struct PhysicType {
    pub shape: Shape,
    pub collision_behavior: CollisionBehavior,
    pub damping: f32,
    pub force: f32,
}
impl specs::Component for PhysicType {
    type Storage = specs::VecStorage<Self>;
}
impl PhysicType {
    pub fn from_yaml(config: &yaml::Yaml) -> Result<PhysicType,String> {
        let hash = try!(config.as_hash().ok_or(""));

        let shape = {
            let vec = try!(try!(hash.get(&yaml::Yaml::from_str("shape")).ok_or("")).as_vec().ok_or(""));
            match try!(vec[0].as_str().ok_or("")) {
                "circle" => Shape::Circle(try!(vec[1].as_f64().ok_or("")) as f32),
                "square" => Shape::Square(try!(vec[1].as_f64().ok_or("")) as f32),
                _ => return Err("".into()),
            }
        };
        let collision_behavior = match try!(try!(hash.get(&yaml::Yaml::from_str("collision_behavior")).ok_or("")).as_str().ok_or("")) {
            "persist" => CollisionBehavior::Persist,
            "bounce" => CollisionBehavior::Bounce,
            "stop" => CollisionBehavior::Stop,
            _ => return Err("".into()),
        };

        let damping = try!(try!(hash.get(&yaml::Yaml::from_str("damping")).ok_or("")).as_f64().ok_or("")) as f32;

        let force = try!(try!(hash.get(&yaml::Yaml::from_str("force")).ok_or("")).as_f64().ok_or("")) as f32;

        Ok(PhysicType {
            shape: shape,
            collision_behavior: collision_behavior,
            damping: damping,
            force: force,
        })
    }
}

#[derive(Debug,Clone)]
pub struct PhysicForce {
    pub direction: f32,
    pub intensity: f32,
}
impl specs::Component for PhysicForce {
    type Storage = specs::VecStorage<Self>;
}
impl PhysicForce {
    pub fn from_yaml(config: &yaml::Yaml) -> Result<PhysicForce,String> {
        let hash = try!(config.as_hash().ok_or(""));

        let direction = try!(try!(hash.get(&yaml::Yaml::from_str("direction")).ok_or("")).as_f64().ok_or("")) as f32;

        let intensity = try!(try!(hash.get(&yaml::Yaml::from_str("intensity")).ok_or("")).as_f64().ok_or("")) as f32;

        Ok(PhysicForce {
            direction: direction,
            intensity: intensity,
        })
    }
}

#[derive(Debug,Clone)]
pub struct Color {
    color: thigra::Color,
}

impl specs::Component for Color {
    type Storage = specs::VecStorage<Self>;
}
impl Color {
    pub fn from_yaml(config: &yaml::Yaml) -> Result<Color,String> {
        let color = match try!(config.as_str().ok_or("")) {
            "base1" => thigra::Color::Base1,
            "base2" => thigra::Color::Base2,
            "base3" => thigra::Color::Base3,
            "base4" => thigra::Color::Base4,
            "base5" => thigra::Color::Base5,
            "yellow" => thigra::Color::Yellow,
            "orange" => thigra::Color::Orange,
            "red" => thigra::Color::Red,
            "magenta" => thigra::Color::Magenta,
            "violet" => thigra::Color::Violet,
            "blue" => thigra::Color::Blue,
            "cyan" => thigra::Color::Cyan,
            "green" => thigra::Color::Green,
            _ => return Err("".into()),
        };

        Ok(Color {
            color: color,
        })
    }
}
