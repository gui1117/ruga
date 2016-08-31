use app;
use components::*;
use specs::Join;
use specs;
use levels;
use config;
use baal;

#[derive(Clone)]
pub struct Portal {
    destination: levels::Level,
}
impl specs::Component for Portal {
    type Storage = specs::VecStorage<Self>;
}
impl Portal {
    pub fn new(destination: levels::Level) -> Self {
        Portal {
            destination: destination,
        }
    }
}

pub struct PortalSystem;
impl specs::System<app::UpdateContext> for PortalSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        let (grid_squares, players, states, portals, entities) = arg.fetch(|world| {
            (
                world.read::<GridSquare>(),
                world.read::<PlayerControl>(),
                world.read::<PhysicState>(),
                world.read::<Portal>(),
                world.entities()
            )
        });

        let mut player_pos = None;
        for (_, state) in (&players, &states).iter() {
            player_pos = Some(state.position);
            break;
        }

        if let Some(player_pos) = player_pos {
            for (portal,entity) in (&portals, &entities).iter() {
                let pos = grid_squares.get(entity).expect("portal expect grid square component").position;
                if (player_pos[0] - pos[0]).powi(2) + (player_pos[1] - pos[1]).powi(2) < 0.01 {
                    baal::effect::short::play(config.entities.portal_snd, baal::effect::listener());
                    context.control_tx.send(app::Control::GotoLevel(portal.destination.clone())).unwrap();
                }
            }
        }
    }
}

