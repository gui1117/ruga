use utils::Into3D;
use snd_effect;
use baal;
use app;
use specs;
use physic;
use specs::Join;
use rand;
use rand::distributions::{IndependentSample, Range};
use app::Effect;
use graphics;

#[derive(Clone,Copy)]
pub enum RifleState {
    ShootOne,
    ShootLots,
    ShootLotsOrOne,
    Rest,
}

pub struct Rifle {
    /// length of the shoot (actual length is lenght - distance)
    pub length: f32,
    pub max_ammo: f32,
    /// time to reach max_ammo
    pub ammo_regen: f32,
    pub damage: f32,
    /// time between to shoot
    pub rate: f32,
    /// number of shoot for ShootLots
    pub lots: u32,
    /// max angle of deviation
    pub deviation: f32,
    /// distance to start the shoot from the position
    pub distance: f32,

    pub aim: f32,
    pub ammo: f32,
    pub state: RifleState,
    pub recovery: f32,
}
impl specs::Component for Rifle {
    type Storage = specs::VecStorage<Self>;
}

pub struct Life(f32);
impl specs::Component for Life {
    type Storage = specs::VecStorage<Self>;
}

pub struct System;

impl specs::System<app::UpdateContext> for System {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        let (mut rifles, mut lives, states, physic_worlds) = arg.fetch(|world| {
            (
                world.write::<Rifle>(),
                world.write::<Life>(),
                world.read::<physic::PhysicState>(),
                world.read::<physic::PhysicWorld>(),
            )
        });

        let physic_world = physic_worlds.get(context.master_entity)
            .expect("master_entity expect physic_world");

        let dt = context.dt as f32;
        let effect_tx = context.effect_tx;

        for (rifle, state) in (&mut rifles, &states).iter() {
            let old_rifle_ammo = rifle.ammo;
            rifle.ammo = rifle.max_ammo.min(rifle.ammo + dt*rifle.max_ammo/rifle.ammo_regen);
            if rifle.ammo.floor() as usize - old_rifle_ammo.floor() as usize == 1 {
                // baal::effect::play(snd_effect::RIFLE_RELOAD,&state.position.into_3d());
            }
            rifle.recovery -= dt;

            if rifle.recovery > 0. { continue; }

            let shoots = match rifle.state {
                RifleState::ShootOne => {
                    if rifle.ammo >= 1. {
                        // baal::effect::play(snd_effect::RIFLE_SHOOT_ONE,&state.position.into_3d());
                        rifle.ammo -= 1.;
                        1
                    } else {
                        // baal::effect::play(snd_effect::RIFLE_SHOOT_ZERO,&state.position.into_3d());
                        0
                    }
                },
                RifleState::ShootLots => {
                    if (rifle.lots as f32) <= rifle.ammo {
                        // baal::effect::play(snd_effect::RIFLE_SHOOT_LOTS,&state.position.into_3d());
                        rifle.ammo -= rifle.lots as f32;
                        rifle.lots
                    } else {
                        // baal::effect::play(snd_effect::RIFLE_SHOOT_ZERO,&state.position.into_3d());
                        0
                    }
                },
                RifleState::ShootLotsOrOne => {
                    if (rifle.lots as f32) <= rifle.ammo {
                        // baal::effect::play(snd_effect::RIFLE_SHOOT_LOTS,&state.position.into_3d());
                        rifle.ammo -= rifle.lots as f32;
                        rifle.lots
                    } else if rifle.ammo >= 1. {
                        rifle.ammo -= 1.;
                        1
                    } else {
                        // baal::effect::play(snd_effect::RIFLE_SHOOT_ZERO,&state.position.into_3d());
                        0
                    }
                },
                RifleState::Rest => { 0 },
            };

            if shoots == 0 { continue; }

            rifle.recovery = shoots as f32 * rifle.rate;

            let mut rng = rand::thread_rng();
            let range = Range::new(-rifle.deviation,rifle.deviation);

            for _ in 0..shoots {
                let angle = rifle.aim + range.ind_sample(&mut rng);
                let origin = [
                    state.position[0] + rifle.distance * angle.cos(),
                    state.position[1] + rifle.distance * angle.sin(),
                ];
                let ray = physic::Ray {
                    origin: origin,
                    angle: angle,
                    length: rifle.length - rifle.distance,
                };

                let mut actual_length = rifle.length;
                physic_world.raycast(&ray, &mut |(entity,start,_)| {
                    if let Some(&mut Life(ref mut life)) = lives.get_mut(entity) {
                        *life -= rifle.damage;
                    } else {
                        actual_length = start;
                    }
                    actual_length = start;
                    true
                });
                effect_tx.send(Effect::Line {
                        origin: origin,
                        angle: angle,
                        length: actual_length,
                        thickness: 5.0,
                        layer: graphics::Layer::Middle,
                        color: graphics::Color::Yellow,
                        persistance: 0.05,
                }).unwrap();
            }
        }
    }
}
