mod character;
mod wall;
mod armory;
mod boid;

pub use self::character::Character;
pub use self::wall::Wall;
pub use self::armory::Armory;
pub use self::boid::Boid;

pub mod group {
    pub const GRENADE: u64 =      0b00000000000000000000000000000001;
    pub const ARMORY: u64 =       0b00000000000000000000000000000010;
    pub const MOVING_WALL: u64 =  0b00000000000000000000000000000100;
    pub const CHARACTER: u64 =    0b00000000000000000000000000001000;
    pub const WALL: u64 =         0b00000000000000000000000000010000;
    pub const BOID: u64 =         0b00000000000000000000000000100000;

    pub const WALL_KIND: u64 =    MOVING_WALL | WALL;
    pub const LIVING: u64 =       CHARACTER | BOID;
    pub const TRIGGER: u64 =      ARMORY;
}
