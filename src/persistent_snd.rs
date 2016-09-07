use specs;
use config;
use baal;
use app;
use components::*;
use specs::Join;
use utils::Into3D;

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
        let (persistent_snds, states) = arg.fetch(|world| {
            (
                world.read::<DynPersistentSnd>(),
                world.read::<PhysicState>(),
            )
        });

        if self.cooldown == 0 {
            if baal::effect::persistent::is_all_mute() {
                baal::effect::persistent::unmute_all();
            }

            self.cooldown = config.general.persistent_snd_cooldown;
            let mut vec: Vec<(usize,Vec<[f32;3]>)> = config.audio.persistent_effects.iter()
                .enumerate()
                .map(|(i,_)| (i,vec!())).collect();

            for (persistent_snd, state) in (&persistent_snds, &states).iter() {
                vec[persistent_snd.id].1.push(state.position.into_3d());
            }

            vec.retain(|&(_,ref v)| !v.is_empty());
            let dyn_persistent_snds: Vec<usize> = vec.iter().map(|&(effect,_)| effect).collect();

            baal::effect::persistent::add_positions_for_all(vec);
            baal::effect::persistent::update_volume_for_all();

            for effect in dyn_persistent_snds {
                baal::effect::persistent::clear_positions(effect);
            }
        } else {
            self.cooldown -= 1;
        }
    }
}
