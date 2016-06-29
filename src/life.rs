use app;
use components::*;
use specs::Join;
use specs;

pub struct Life {
    alive: bool,
    //TODO die_snd: usize,
}

impl specs::Component for Life {
    type Storage = specs::VecStorage<Self>;
}

impl Life {
    pub fn new() -> Self {
        Life {
            alive: true,
            // die_snd: die_snd,
        }
    }
    pub fn kill(&mut self) {
        self.alive = false;
        //TODO play die_snd
    }
}

pub struct LifeSystem;
impl specs::System<app::UpdateContext> for LifeSystem {
    fn run(&mut self, arg: specs::RunArg, _context: app::UpdateContext) {
        let (lives, entities) = arg.fetch(|world| {
            (
                world.read::<Life>(),
                world.entities(),
            )
        });
        for (life, entity) in (&lives, &entities).iter() {
            if !life.alive {
                arg.delete(entity);
            }
        }
    }
}

pub struct Killer {
    kamikaze: bool,
    group: u32,
    //TODO kill_snd
}
impl specs::Component for Killer {
    type Storage = specs::VecStorage<Self>;
}

pub struct KillerSystem;
impl specs::System<app::UpdateContext> for KillerSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        let (mut lives, states, types, physic_worlds, killers, entities) = arg.fetch(|world| {
            (
                world.write::<Life>(),
                world.read::<PhysicState>(),
                world.read::<PhysicType>(),
                world.read::<PhysicWorld>(),
                world.read::<Killer>(),
                world.entities(),
            )
        });

        let physic_world = physic_worlds.get(context.master_entity)
            .expect("master_entity expect physic_world component");

        for (killer, state, typ, entity) in (&killers, &states, &types, &entities).iter() {
            let mut kill = false;
            physic_world.apply_on_shape(&state.position, killer.group, &typ.shape, &mut |other_entity,_| {
                if let Some(mut life) = lives.get_mut(*other_entity) {
                    life.kill();
                    kill = true;
                }
            });
            if kill && killer.kamikaze {
                lives.get_mut(entity).expect("killer kamikaze expect life component").kill();
            }
        }
    }
}

pub struct Ball {
    origin: [f32;2],
}
impl specs::Component for Ball {
    type Storage = specs::VecStorage<Self>;
}

pub struct BallSystem;
impl specs::System<app::UpdateContext> for BallSystem {
    fn run(&mut self, arg: specs::RunArg, _context: app::UpdateContext) {
        let (mut states, balls, triggers) = arg.fetch(|world| {
            (
                world.write::<PhysicState>(),
                world.read::<Ball>(),
                world.read::<PhysicTrigger>(),
            )
        });
        for (ball, trigger, mut state) in (&balls, &triggers, &mut states).iter() {
            if trigger.active {
                state.position = ball.origin;
                state.velocity = [0.,0.];
                state.acceleration = [0.,0.];
            }
        }
    }
}

