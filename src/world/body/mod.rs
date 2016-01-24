mod character;
mod wall;
mod body;
mod monster;
mod boids;
pub mod traits;

pub use self::character::Character;
pub use self::wall::Wall;
pub use self::body::Body;
pub use self::monster::Monster;
pub use self::boids::Boid;
pub use self::traits::BodyTrait;

#[derive(Clone)]
pub enum CollisionBehavior {
    Persist,
    Stop,
    Bounce
}

#[derive(Clone,PartialEq)]
pub enum BodyType {
    Wall,
    Character,
    Monster,
    Boid,
}

//pub struct BodySnapshot;
