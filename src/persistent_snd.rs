use specs;
use config;
use baal;
use app;
use components::*;
use specs::Join;
use utils::Into3D;

pub struct StaticPersistentSnd {
    id: usize,
}
impl specs::Component for StaticPersistentSnd {
    type Storage = specs::VecStorage<Self>;
}
impl StaticPersistentSnd {
    pub fn new(id: usize) -> Self {
        StaticPersistentSnd {
            id: id,
        }
    }
}

pub struct DynPersistentSnd {
    id: usize,
}
impl specs::Component for DynPersistentSnd {
    type Storage = specs::VecStorage<Self>;
}
impl DynPersistentSnd {
    pub fn new(id: usize) -> Self {
        DynPersistentSnd {
            id: id,
        }
    }
}

pub struct PersistentSndSystem {
    cooldown: usize,
}
impl Default for PersistentSndSystem {
    fn default() -> Self {
        PersistentSndSystem {
            cooldown: 0,
        }
    }
}

impl specs::System<app::UpdateContext> for PersistentSndSystem {
    fn run(&mut self, arg: specs::RunArg, _context: app::UpdateContext) {
        let (dyn_persistent_snds, states) = arg.fetch(|world| {
            (
                world.read::<DynPersistentSnd>(),
                world.read::<PhysicState>(),
            )
        });

        if self.cooldown == 0 {
            self.cooldown = config.general.persistent_snd_cooldown;
            let mut vec: Vec<(usize,Vec<[f32;3]>)> = config.audio.persistent_effects.iter()
                .enumerate()
                .map(|(i,_)| (i,vec!())).collect();

            let mut dyn_persistent_snd_ids = vec!();
            for (dyn_persistent_snd, state) in (&dyn_persistent_snds, &states).iter() {
                vec[dyn_persistent_snd.id].1.push(state.position.into_3d());
                dyn_persistent_snd_ids.push(dyn_persistent_snd.id);
            }

            vec.retain(|&(_,ref v)| !v.is_empty());

            baal::effect::persistent::add_positions_for_all(vec);
            baal::effect::persistent::update_volume_for_all();

            for id in dyn_persistent_snd_ids {
                baal::effect::persistent::clear_positions(id);
            }
        } else {
            self.cooldown -= 1;
        }
    }
}

pub fn reset_static_persistent_snd(world: &specs::World) {
    let states =  world.read::<PhysicState>();
    let static_persistent_snds = world.read::<StaticPersistentSnd>();
    let entities = world.entities();

    for (static_persistent_snd, entity) in (&static_persistent_snds, &entities).iter() {
        let state = states.get(entity).expect("static persistent snd expect state component");
        baal::effect::persistent::add_position(
            static_persistent_snd.id,
            state.position.into_3d());
    }
}
