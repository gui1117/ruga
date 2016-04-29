use specs;

#[derive(Debug,Clone)]
pub enum Shape {
    Circle(f32),
    Square(f32),
}

#[derive(Debug,Clone)]
pub enum CollisionBehavior {
    Bounce,
    Back,
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

#[derive(Debug,Clone)]
pub struct PhysicForce {
    pub direction: f32,
    pub intensity: f32,
}
impl specs::Component for PhysicForce {
    type Storage = specs::VecStorage<Self>;
}

// #[derive(Debug,Clone)]
// pub struct Color {
//     color: thigra::Color,
// }

// impl specs::Component for Color {
//     type Storage = specs::VecStorage<Self>;
// }
// impl Color {
//     pub fn from_yaml(config: &yaml::Yaml) -> Result<Color,String> {
//         let color = match try!(config.as_str().ok_or("")) {
//             "base1" => thigra::Color::Base1,
//             "base2" => thigra::Color::Base2,
//             "base3" => thigra::Color::Base3,
//             "base4" => thigra::Color::Base4,
//             "base5" => thigra::Color::Base5,
//             "yellow" => thigra::Color::Yellow,
//             "orange" => thigra::Color::Orange,
//             "red" => thigra::Color::Red,
//             "magenta" => thigra::Color::Magenta,
//             "violet" => thigra::Color::Violet,
//             "blue" => thigra::Color::Blue,
//             "cyan" => thigra::Color::Cyan,
//             "green" => thigra::Color::Green,
//             _ => return Err("".into()),
//         };

//         Ok(Color {
//             color: color,
//         })
//     }
// }

