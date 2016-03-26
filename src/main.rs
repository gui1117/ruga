struct fire_weapon_type {
    pub ammo: u64,
    pub rate: f64,
    pub projectile: u64,
    pub apeture: f64,
    pub range: f64,
    pub damage: f64,
}

struct fire_weapon_state {
    pub ammo: u64,
    pub aim: f64,
    pub shoot: bool,
    pub recovery: f64,
}

struct bladed_weapon_type {
    pub stamina: f64,
    pub range: f64,
    pub aperture: f64,
    pub damage: f64,
    pub rate: f64,
}

struct bladed_weapon_state {
    pub recovery: f64,
    pub stamina: f64,
}

struct fire_weapon_processor;

fn main() {
}
