use app;
use components::*;
use specs::Join;
use specs;
use conf::config;

#[derive(Clone,Default)]
pub struct Portal;
impl specs::Component for Portal {
    type Storage = specs::NullStorage<Self>;
}

pub struct PortalSystem;
impl specs::System<app::UpdateContext> for PortalSystem {
    fn run(&mut self, arg: specs::RunArg, _context: app::UpdateContext) {
        let (players, states, portals, entities) = arg.fetch(|world| {
            (
                world.read::<PlayerControl>(),
                world.read::<PhysicState>(),
                world.read::<Portal>(),
                world.entities(),
            )
        });

        let mut player_pos = None;
        for (_, state) in (&players, &states).iter() {
            player_pos = Some(state.position);
            break;
        }

        if let Some(player_pos) = player_pos {
            for (portal, entity) in (&portals, &entities).iter() {
                let state = states.get(entity).expect("portal component expect physic state component");

                if (player_pos[0] - state.position[0]).powi(2) + (player_pos[1] - state.position[1]).powi(2) < config.physic.unit {
                    //TODO end level
                }
            }
        }
    }
}

