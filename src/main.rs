pub struct FireWeaponType {
    pub ammo: u64,
    pub rate: f64,
    pub projectile: u64,
    pub apeture: f64,
    pub range: f64,
    pub damage: f64,
}

pub struct FireWeaponState {
    pub ammo: u64,
    pub aim: f64,
    pub shoot: bool,
    pub recovery: f64,
}

pub struct BladedWeaponType {
    pub stamina_max: f64,
    pub stamina_rate: f64,
    pub range: f64,
    pub aperture: f64,
    pub damage: f64,
    pub rate: f64,
}

pub struct BladedWeaponState {
    pub recovery: f64,
    pub stamina: f64,
    pub attack: bool,
}

pub struct FireWeaponShoot;

impl FireWeaponShoot {
    pub fn process(&mut self, dt: f64, w_state: &mut FireWeaponState, w_type: &FireWeaponType) {
        w_state.recovery -= dt;
        if w_state.shoot && w_state.ammo != 0 && w_state.recovery <= 0. {
            w_state.recovery += w_type.rate;
            w_state.ammo -= 1;
            //TODO shoot
        } else {
            w_state.recovery = (w_state.recovery).max(0.);
        }
    }
}

pub struct BladedWeaponAttack;

impl BladedWeaponAttack {
    pub fn process(&mut self, dt: f64, w_state: &mut BladedWeaponState, w_type: &BladedWeaponType) {
        w_state.recovery -= dt;
        w_state.stamina = (w_state.stamina + dt).max(w_type.stamina_max);
        if w_state.attack && w_state.stamina > 0. && w_state.recovery <= 0. {
            w_state.recovery += w_type.rate;
            w_state.stamina -= w_type.stamina_rate;
            //TODO attack
        } else {
            w_state.recovery = (w_state.recovery).max(0.);
        }
    }
}

pub struct PhysicState {
    pub position: [f64;2],
    pub velocity: [f64;2],
    pub acceleration: [f64;2],
}

pub struct PhysicType {
    // pub shape:
    // pub collision_behavior:
    pub damping: f64,
    pub force: f64,
}

pub struct PhysicForce {
    pub direction: f64,
    pub intensity: f64,
}

pub struct Collision {
    pub damage: f64
}

pub struct PhysicStep;

impl PhysicStep {
    pub fn process(&mut self, dt: f64, p_state: &mut PhysicState, p_collision: &mut Collision, p_type: &PhysicType, p_forces: &PhysicForce) {
    }
}

fn main() {
}
