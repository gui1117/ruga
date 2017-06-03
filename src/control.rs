use app;
use components::*;
use resource::*;
use specs::Join;
use specs;
use config;
use rand;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug, Clone, Default)]
pub struct PlayerControl;
impl specs::Component for PlayerControl {
    type Storage = specs::NullStorage<Self>;
}
pub struct PlayerSystem {
    restart_cooldown: Option<f32>,
}
impl Default for PlayerSystem {
    fn default() -> Self {
        PlayerSystem {
            restart_cooldown: None,
        }
    }
}
impl specs::System<app::UpdateContext> for PlayerSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        let (players, states, entities) = arg.fetch(|world| {
            (
                world.read::<PlayerControl>(),
                world.read::<PhysicState>(),
                world.entities(),
            )
        });

        if let Some((_, entity)) = (&players, &entities).iter().nth(0) {
            let state = states.get(entity).expect("playercontrol expect state component");
            // baal::effect::set_listener(state.position.into_3d());
            self.restart_cooldown = Some(config.entities.char_restart);
        } else {
            self.restart_cooldown = if let Some(cooldown) = self.restart_cooldown {
                if cooldown > 0. {
                    Some(cooldown - context.dt)
                } else {
                    context.control_tx.send(app::Control::ResetLevel).unwrap();
                    None
                }
            } else {
                None
            };
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TowardPlayerControl;
impl specs::Component for TowardPlayerControl {
    type Storage = specs::NullStorage<Self>;
}

pub struct TowardPlayerSystem;
impl specs::System<app::UpdateContext> for TowardPlayerSystem {
    fn run(&mut self, arg: specs::RunArg, _context: app::UpdateContext) {
        let (toward_players, players, mut forces, states, entities) = arg.fetch(|world| {
            (
                world.read::<TowardPlayerControl>(),
                world.read::<PlayerControl>(),
                world.write::<PhysicForce>(),
                world.read::<PhysicState>(),
                world.entities(),
            )
        });

        let mut player_pos = None;
        for (_, state) in (&players, &states).iter() {
            player_pos = Some(state.position);
            break;
        }

        if let Some(player_pos) = player_pos {
            for (_, entity) in (&toward_players, &entities).iter() {
                let state = states.get(entity).expect("toward player component expect state component");
                let force = forces.get_mut(entity).expect("toward player component expect force component");

                let pos = state.position;
                force.direction = (player_pos[1] - pos[1]).atan2(player_pos[0] - pos[0]);
            }
        }
    }
}

pub struct MonsterControl {
    next_lookup: f32,
}
impl specs::Component for MonsterControl {
    type Storage = specs::VecStorage<Self>;
}
impl MonsterControl {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let range = Range::new(0., config.entities.monster_vision_time);
        MonsterControl {
            next_lookup: range.ind_sample(&mut rng),
        }
    }
}

pub struct MonsterSystem;
impl specs::System<app::UpdateContext> for MonsterSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        let (mut monsters, players, mut forces, states, physic_world, entities) = arg.fetch(|world| {
            (
                world.write::<MonsterControl>(),
                world.read::<PlayerControl>(),
                world.write::<PhysicForce>(),
                world.read::<PhysicState>(),
                world.read_resource::<PhysicWorld>(),
                world.entities(),
            )
        });

        let mut player_pos = None;
        for (_, state) in (&players, &states).iter() {
            player_pos = Some(state.position);
            break;
        }

        if let Some(player_pos) = player_pos {
            for (mut monster, entity) in (&mut monsters, &entities).iter() {
                let state = states.get(entity).expect("monster expect state component");
                let force = forces.get_mut(entity).expect("monster expect force component");

                let pos = state.position;
                let angle = (player_pos[1] - pos[1]).atan2(player_pos[0] - pos[0]);

                force.direction = angle;

                monster.next_lookup -= context.dt;

                if monster.next_lookup <= 0. {
                    let length = ((player_pos[1] - pos[1]).powi(2) + (player_pos[0] - pos[0]).powi(2)).sqrt();
                    let ray = Ray {
                        origin: pos,
                        angle: angle,
                        length: length,
                        mask: config.entities.monster_vision_mask.0,
                    };

                    let mut player_visible = false;
                    physic_world.raycast(&ray, &mut |(other_entity, _, _)| {
                        if players.get(other_entity).is_some() {
                            player_visible = true;
                        }
                        true
                    });
                    if player_visible {
                        force.intensity = 1.;
                    } else {
                        force.intensity = 0.;
                    }

                    monster.next_lookup = config.entities.monster_vision_time;
                }
            }
        }
    }
}

