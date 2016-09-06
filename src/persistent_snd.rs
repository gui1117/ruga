use specs;
use config;
use baal;
use app;
use components::*;
use specs::Join;
use utils::Into3D;

pub struct PersistentSnd {
    id: usize,
}
impl specs::Component for PersistentSnd {
    type Storage = specs::VecStorage<Self>;
}
impl PersistentSnd {
    pub fn new(id: usize) -> Self {
        PersistentSnd {
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
                world.read::<PersistentSnd>(),
                world.read::<PhysicState>(),
            )
        });

        if self.cooldown == 0 {
            self.cooldown = config.general.persistent_snd_cooldown;
            let mut vec: Vec<(usize,Vec<[f32;3]>)> = config.audio.persistent_effects.iter()
                .enumerate()
                .map(|(i,_)| (i,vec!())).collect();

            for (persistent_snd, state) in (&persistent_snds, &states).iter() {
                vec[persistent_snd.id].1.push(state.position.into_3d());
            }

            if baal::effect::persistent::is_all_mute() {
                baal::effect::persistent::unmute_all();
            }
            baal::effect::persistent::add_positions_for_all(vec);
            baal::effect::persistent::update_volume_for_all();
            baal::effect::persistent::clear_positions_for_all();
        } else {
            self.cooldown -= 1;
        }
    }
}
