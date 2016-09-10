use app;
use graphics;
use components::*;
use specs::Join;
use specs;
use utils::Into3D;
use baal;
use config;
use std::sync::Arc;

pub struct Life {
    alive: bool,
    die_snd: usize,
}

impl specs::Component for Life {
    type Storage = specs::VecStorage<Self>;
}

impl Life {
    pub fn new(die_snd: usize) -> Self {
        Life {
            alive: true,
            die_snd: die_snd,
        }
    }
    pub fn kill(&mut self) {
        self.alive = false;
    }
}

pub struct LifeSystem;
impl specs::System<app::UpdateContext> for LifeSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        let (mut lives, mut states, entities) = arg.fetch(|world| {
            (
                world.write::<Life>(),
                world.write::<PhysicState>(),
                world.entities(),
            )
        });
        for (life, entity) in (&mut lives, &entities).iter() {
            if !life.alive {
                let state = states.get_mut(entity).expect("life expect state component");

                for &angle in &config.effect.angles {
                    let origin = [
                        state.position[0]+config.effect.inner_length*angle.cos(),
                        state.position[1]+config.effect.inner_length*angle.sin()
                    ];

                    context.effect_tx.send(app::Effect::Line {
                        origin: origin,
                        length: config.effect.length,
                        angle: angle,
                        persistance: config.effect.persistance,
                        thickness: config.effect.thickness,
                        layer: graphics::Layer::Ceil,
                        color: config.effect.color,
                    }).unwrap();
                }

                baal::effect::short::play(life.die_snd,state.position.into_3d());
                arg.delete(entity);
            }
        }
    }
}

pub struct Killer {
    pub kamikaze: bool,
    pub mask: u32,
    pub kill_snd: usize
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
            physic_world.apply_on_shape(&state.position, killer.mask, &typ.shape, &mut |other_entity,_| {
                if let Some(life) = lives.get_mut(*other_entity) {
                    baal::effect::short::play(killer.kill_snd,state.position.into_3d());
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
    _arc: Arc<()>,
    snd_timer: f32,
}
impl specs::Component for Ball {
    type Storage = specs::VecStorage<Self>;
}
impl Ball {
    pub fn new(arc: Arc<()>) -> Ball {
        Ball {
            _arc: arc,
            snd_timer: 1.0,
        }
    }
}

pub struct BallSystem;
impl specs::System<app::UpdateContext> for BallSystem {
    fn run(&mut self, arg: specs::RunArg, _context: app::UpdateContext) {
        use std::ops::Mul;

        let (mut lives, states, mut balls, triggers, entities) = arg.fetch(|world| {
            (
                world.write::<Life>(),
                world.read::<PhysicState>(),
                world.write::<Ball>(),
                world.read::<PhysicTrigger>(),
                world.entities(),
            )
        });
        for (ball, entity) in (&mut balls, &entities).iter() {
            let trigger = triggers.get(entity).expect("ball component expect trigger component");
            let life = lives.get_mut(entity).expect("ball component expect life component");
            let state = states.get(entity).expect("ball component expect life component");

            ball.snd_timer -= (state.velocity[0].powi(2)+state.velocity[1].powi(2))
                .sqrt()
                .mul(config.entities.ball_vel_snd_coef);

            if ball.snd_timer <= 0. {
                ball.snd_timer += 1.0;
                baal::effect::short::play(config.entities.ball_vel_snd,state.position.into_3d());
            }

            if trigger.active {
                life.kill();
            }
        }
    }
}

pub struct Column {
    spawn_snd: usize,
    cooldown: Option<f32>,
    arc: Arc<()>,
}
impl specs::Component for Column {
    type Storage = specs::VecStorage<Self>;
}
impl Column {
    pub fn new(snd: usize) -> Column {
        Column {
            spawn_snd: snd,
            cooldown: Some(config.entities.column_cooldown),
            arc: Arc::new(()),
        }
    }
}
pub struct ColumnSystem;
impl specs::System<app::UpdateContext> for ColumnSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        let (mut columns, states, entities) = arg.fetch(|world| {
            (
                world.write::<Column>(),
                world.read::<PhysicState>(),
                world.entities(),
            )
        });
        for (column, entity) in (&mut columns, &entities).iter() {
            column.cooldown = if let Some(cooldown) = column.cooldown {
                if cooldown > 0. {
                    Some(cooldown - context.dt as f32)
                } else {
                    let state = states.get(entity).expect("column component expect state component");
                    context.control_tx.send(app::Control::CreateBall(state.position,column.arc.clone())).unwrap();
                    baal::effect::short::play(column.spawn_snd,state.position.into_3d());
                    None
                }
            } else if let Some(_) = Arc::get_mut(&mut column.arc) {
                Some(config.entities.column_cooldown)
            } else {
                column.cooldown
            }
        }
    }
}
