use app;
use components::*;
use specs::Join;
use specs;
use config;
use rand;
use rand::distributions::{IndependentSample, Range};

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

pub struct TowardPlayerSystem;
impl specs::System<app::UpdateContext> for TowardPlayerSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        let (toward_players, players, mut forces, states) = arg.fetch(|world| {
            (
                world.read::<TowardPlayerControl>(),
                world.read::<PlayerControl>(),
                world.write::<PhysicForce>(),
                world.read::<PhysicState>(),
            )
        });

        let mut player_pos = None;
        for (_, state) in (&players, &states).iter() {
            player_pos = Some(state.position);
            break;
        }

        if let Some(player_pos) = player_pos {
            for (_, state, mut force) in (&toward_players, &states, &mut forces).iter() {
                let pos = state.position;
                force.direction = (player_pos[1] - pos[1]).atan2(player_pos[0] - pos[0]);
            }
        }
    }
}

pub struct MonsterControl {
    state: usize,
    next_lookup: f32,
}
impl specs::Component for MonsterControl {
    type Storage = specs::VecStorage<Self>;
}
impl MonsterControl {
    pub fn new() -> Self {
        MonsterControl {
            state: 0,
            next_lookup: 0.,
        }
    }
}

pub struct MonsterSystem;
impl specs::System<app::UpdateContext> for MonsterSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        use std::usize;

        let (mut monsters, players, mut forces, states, physic_worlds) = arg.fetch(|world| {
            (
                world.write::<MonsterControl>(),
                world.read::<PlayerControl>(),
                world.write::<PhysicForce>(),
                world.read::<PhysicState>(),
                world.read::<PhysicWorld>(),
            )
        });
        let physic_world = physic_worlds.get(context.master_entity)
            .expect("master_entity expect physic_world component");

        let mut player_pos = None;
        for (_, state) in (&players, &states).iter() {
            player_pos = Some(state.position);
            break;
        }

        let mut rng = rand::thread_rng();
        if let Some(player_pos) = player_pos {
            for (mut monster, state, mut force) in (&mut monsters, &states, &mut forces).iter() {
                monster.next_lookup -= context.dt as f32;

                if monster.next_lookup <= 0. {
                    let pos = state.position;
                    let angle = (player_pos[1] - pos[1]).atan2(player_pos[0] - pos[0]);
                    let length = ((player_pos[1] - pos[1]).powi(2) + (player_pos[1] - pos[1]).powi(2)).sqrt();
                    let ray = Ray {
                        origin: pos,
                        angle: angle,
                        length: length,
                        group: config.entity.monster_vision_group,
                    };

                    let mut player_visible = false;
                    physic_world.raycast(&ray, &mut |(entity,_,_)| {
                        if players.get(entity).is_some() {
                            player_visible = true;
                            return true;
                        }
                        false
                    });
                    if player_visible {
                        if monster.state != config.entity.monster_velocities.len() {
                            monster.state += 1;
                        }
                        force.direction = angle;
                    } else if monster.state != 0 {
                        monster.state -= 1;
                    }
                    force.intensity = config.entity.monster_velocities[monster.state];

                    let range = Range::new(0.,config.entity.monster_ranges[monster.state]);
                    monster.next_lookup = range.ind_sample(&mut rng);
                }
            }
        }
    }
}

