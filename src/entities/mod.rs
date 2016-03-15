pub mod character;
mod wall;
mod armory;
mod boid;
mod spider;
mod grenade;
mod burning_wall;

pub use self::character::{Character,CharacterManager};
pub use self::wall::Wall;
pub use self::armory::Armory;
pub use self::boid::Boid;
pub use self::spider::Spider;
pub use self::grenade::Grenade;
pub use self::burning_wall::BurningWall;

pub mod group {
    pub const GRENADE: u64 =      0b00000000000000000000000000000001;
    pub const ARMORY: u64 =       0b00000000000000000000000000000010;
    pub const BURNING_WALL: u64 = 0b00000000000000000000000000000100;
    pub const CHARACTER: u64 =    0b00000000000000000000000000001000;
    pub const WALL: u64 =         0b00000000000000000000000000010000;
    pub const BOID: u64 =         0b00000000000000000000000000100000;
    pub const SPIDER: u64 =       0b00000000000000000000000001000000;

    pub const WALL_KIND: u64 =    WALL|BURNING_WALL;
    pub const LIVING: u64 =       CHARACTER | BOID | SPIDER;
    pub const TRIGGER: u64 =      ARMORY;
    pub const BULLET: u64 =       GRENADE;
}
