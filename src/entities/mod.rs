mod character;
mod wall;
mod armory;
mod boid;
mod spider;
mod snake;
mod grenade;

pub use self::character::Character;
pub use self::wall::Wall;
pub use self::armory::Armory;
pub use self::boid::Boid;
pub use self::spider::Spider;
pub use self::snake::Snake;
pub use self::grenade::Grenade;

pub mod group {
    pub const GRENADE: u64 =      0b00000000000000000000000000000001;
    pub const ARMORY: u64 =       0b00000000000000000000000000000010;
    pub const CHARACTER: u64 =    0b00000000000000000000000000001000;
    pub const WALL: u64 =         0b00000000000000000000000000010000;
    pub const BOID: u64 =         0b00000000000000000000000000100000;
    pub const SPIDER: u64 =       0b00000000000000000000000001000000;
    pub const SNAKE: u64 =        0b00000000000000000000000010000000;

    pub const WALL_KIND: u64 =    WALL;
    pub const LIVING: u64 =       CHARACTER | BOID | SPIDER | SNAKE;
    pub const TRIGGER: u64 =      ARMORY;
    pub const BULLET: u64 =       GRENADE;
}
