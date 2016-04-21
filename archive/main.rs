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

struct World;

impl FireWeaponShoot {
    pub fn process(&mut self, dt: f64, sh: &mut Schedule) {//w_state: &mut FireWeaponState, w_type: &FireWeaponType) {
        let w_state = sh.get_mut::<FireWeaponState>().unwrap();
        let w_weapon = sh.get_mut::<FireWeaponType>().unwrap();
        // // iterate over w_state and w_type
        // w_state.recovery -= dt;
        // if w_state.shoot && w_state.ammo != 0 && w_state.recovery <= 0. {
        //     w_state.recovery += w_type.rate;
        //     w_state.ammo -= 1;
        //     //TODO shoot
        // } else {
        //     w_state.recovery = (w_state.recovery).max(0.);
        // }
    }
}

pub struct BladedWeaponAttack;

impl BladedWeaponAttack {
    pub fn process(&mut self, dt: f64, schedule: &mut Schedule) {//w_state: &mut BladedWeaponState, w_type: &BladedWeaponType) {
        // w_state.recovery -= dt;
        // w_state.stamina = (w_state.stamina + dt).max(w_type.stamina_max);
        // if w_state.attack && w_state.stamina > 0. && w_state.recovery <= 0. {
        //     w_state.recovery += w_type.rate;
        //     w_state.stamina -= w_type.stamina_rate;
        //     //TODO attack
        // } else {
        //     w_state.recovery = (w_state.recovery).max(0.);
        // }
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
    pub fn process(&mut self, dt: f64, schedule: &mut Schedule) {//p_state: &mut PhysicState, p_collision: &mut Collision, p_type: &PhysicType, p_forces: &PhysicForce) {
        // compute new acceleration, velocity and position
        // resolve collision
    }
}

pub struct Schedule;
impl Schedule {
      pub fn get<T>(&mut self) -> Result<FutureData<&T>,()> {
          Err(())
      }
      pub fn get_mut<T>(&mut self) -> Result<FutureData<&mut T>,()> {
          Err(())
      }
}
pub struct FutureData<T> {
    data: T,
}
impl<T> FutureData<T> {
      pub fn with<Q>(self, other: FutureData<Q>) -> FutureData<(T, Q)> {
          FutureData {
              data: (self.data,other.data),
          }
      }
      pub fn iter<F: FnMut(T)>(self, func: F) {}
}

fn main() {
}
