use app;
use physic::IntoGrid;
use components::*;
use specs::Join;
use specs;
use conf::config;

#[derive(Clone,Default)]
pub struct Portal {
    destination: String,
    position: [f32;2],
}
impl specs::Component for Portal {
    type Storage = specs::VecStorage<Self>;
}
impl Portal {
    pub fn new<T: IntoGrid>(pos: T, destination: String) -> Self {
        Portal {
            destination: destination,
            position: pos.into_grid(),
        }
    }
}

pub struct PortalSystem;
impl specs::System<app::UpdateContext> for PortalSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        let (players, states, portals) = arg.fetch(|world| {
            (
                world.read::<PlayerControl>(),
                world.read::<PhysicState>(),
                world.read::<Portal>(),
            )
        });

        let mut player_pos = None;
        for (_, state) in (&players, &states).iter() {
            player_pos = Some(state.position);
            break;
        }

        if let Some(player_pos) = player_pos {
            for portal in (&portals).iter() {
                if (player_pos[0] - portal.position[0]).powi(2) + (player_pos[1] - portal.position[1]).powi(2) < (config.physic.unit/2.).powi(2) {
                    context.control_tx.send(app::Control::GotoLevel(portal.destination.clone())).unwrap();
                }
            }
        }
    }
}

