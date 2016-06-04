use specs;
use physic;
use specs::Join;

pub struct Rifle {
    pub rate: f32,
    pub length: f32,
    pub damage: f32,
    pub shoot: bool,
    pub recovery: f32,
    pub ammo: u32,
    pub aim: f32,
}
impl specs::Component for Rifle {
    type Storage = specs::VecStorage<Self>;
}

pub struct Life(f32);
impl specs::Component for Life {
    type Storage = specs::VecStorage<Self>;
}

pub struct System;

impl specs::System<super::UpdateContext> for System {
    fn run(&mut self, arg: specs::RunArg, context: super::UpdateContext) {
        // let physic_world = context.physic_world.borrow();
        let (mut rifles, states) = arg.fetch(|world| {
            (
                world.write::<Rifle>(),
                world.read::<physic::PhysicState>(),
            )
        });
        // for (rifle, state) in (&mut rifles, &states).iter() {
        //     let ray = physic::Ray {
        //         origin: state.position,
        //         angle: rifle.aim,
        //         length: rifle.length,
        //     };
        //     physic_world.raycast(&ray, &mut |(entity,start,end)| {
        //         false
        //     });
        // }
    }
}
// pub fn process(&mut self, dt: f64, sh: &mut Schedule) {//w_state: &mut FireWeaponState, w_type: &FireWeaponType) {
//     let w_state = sh.get_mut::<FireWeaponState>().unwrap();
//     let w_weapon = sh.get_mut::<FireWeaponType>().unwrap();
    // // iterate over w_state and w_type
    // w_state.recovery -= dt;
    // if w_state.shoot && w_state.ammo != 0 && w_state.recovery <= 0. {
    //     w_state.recovery += w_type.rate;
    //     w_state.ammo -= 1;
    //     //TODO shoot
    // } else {
    //     w_state.recovery = (w_state.recovery).max(0.);
    // }
// }

