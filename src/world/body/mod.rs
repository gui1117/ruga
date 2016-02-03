pub mod character;
pub mod wall;
pub mod body;
pub mod monster;
pub mod boids;
pub mod traits;
pub mod snake;
pub mod grenade;

pub use self::grenade::Grenade;
pub use self::character::Character;
pub use self::wall::Wall;
pub use self::body::Body;
pub use self::monster::Monster;
pub use self::snake::Snake;
pub use self::boids::Boid;
pub use self::traits::BodyTrait;

#[derive(Clone)]
pub enum CollisionBehavior {
    Persist,
    Stop,
    Bounce,
    Random,
}

#[derive(Clone,PartialEq)]
pub enum BodyType {
    Wall,
    Character,
    Monster,
    Boid,
    Snake,
    Grenade,
    Armory,
}

//pub struct BodySnapshot;
