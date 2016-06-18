use specs;

#[derive(Debug,Clone,Default)]
pub struct PlayerControl;
impl specs::Component for PlayerControl {
    type Storage = specs::NullStorage<Self>;
}

#[derive(Debug,Clone,Default)]
pub struct TowardPlayerControl;
impl specs::Component for TowardPlayerControl {
    type Storage = specs::NullStorage<Self>;
}

// pub struct System;

// impl specs::System<app::UpdateContext> for System {
//     fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
//         let (mut rifles, mut lives, states, physic_worlds) = arg.fetch(|world| {
//             (
//                 world.write::<Rifle>(),
//                 world.write::<Life>(),
//                 world.read::<physic::PhysicState>(),
//                 world.read::<physic::PhysicWorld>(),
//             )
//         });

//         let physic_world = physic_worlds.get(context.master_entity)
//             .expect("master_entity expect physic_world");
//     }
// }

