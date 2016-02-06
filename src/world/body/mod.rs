pub mod character;
pub mod wall;
pub mod moving_wall;
pub mod body;
pub mod boids;
pub mod armory;
pub mod traits;
//pub mod snake;
pub mod grenade;

pub use self::grenade::Grenade;
pub use self::armory::Armory;
pub use self::moving_wall::MovingWall;
pub use self::character::Character;
pub use self::wall::Wall;
pub use self::body::Body;
//pub use self::snake::Snake;
pub use self::boids::Boid;
pub use self::traits::BodyTrait;

#[derive(Clone)]
pub enum CollisionBehavior {
    Persist,
    Stop,
    Bounce,
    Random,
}

//GROUP
pub const GRENADE_GROUP: u32 =      0b00000000000000000000000000000001;
pub const ARMORY_GROUP: u32 =       0b00000000000000000000000000000010;
pub const MOVING_WALL_GROUP: u32 =  0b00000000000000000000000000000100;
pub const CHARACTER_GROUP: u32 =    0b00000000000000000000000000001000;
pub const WALL_GROUP: u32 =         0b00000000000000000000000000010000;
pub const BOID_GROUP: u32 =         0b00000000000000000000000000100000;

#[derive(Clone,PartialEq)]
pub enum BodyType {
    Wall,
    MovingWall,
    Character,
    Boid,
    //Snake,
    Grenade,
    Armory,
}

//pub struct BodySnapshot;
