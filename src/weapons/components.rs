use super::*;

impl_component! {
    Aim: HashMapStorage,
    Shoot: NullStorage,
    Weapon: HashMapStorage,
    NextWeapon: HashMapStorage,
}

#[derive(Clone)]
pub struct Aim(pub f32);

#[derive(Clone,Copy,Default)]
pub struct Shoot;

#[derive(Clone)]
pub struct Weapon {
    pub reload_factor: f32,
    pub setup_factor: f32,
    pub setdown_factor: f32,
    pub state: State,
    pub kind: Kind,
}

#[derive(Clone)]
pub struct NextWeapon(pub Weapon);
