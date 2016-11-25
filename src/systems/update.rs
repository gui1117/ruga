use app::UpdateContext;
use fnv::FnvHashMap;
use app;
use specs;
use physics::{EntityInformation, Resolution, ShapeCast, Collision, CollisionBehavior};
use components::*;
use resources::*;
use specs::Join;
use utils::math::*;

pub fn add_systems(planner: &mut ::specs::Planner<UpdateContext>) {
    planner.add_system(AnchorSystem, "anchor", 2);
    planner.add_system(SpringSystem, "spring", 1);
    planner.add_system(PhysicSystem, "physic", 0);
}

pub struct AnchorSystem;
impl specs::System<app::UpdateContext> for AnchorSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        let (anchors, mut states, orientations, entities) = arg.fetch(|world| {
            (
                world.read::<Anchor>(),
                world.write::<PhysicState>(),
                world.read::<Orientation>(),
                world.entities(),
            )
        });
        for (anchor, entity) in (&anchors, &entities).iter() {
            let anchor_state = states.get(anchor.anchor)
                .expect("anchor anchor expect a state").clone();
            let mut entity_state = states.get_mut(entity)
                .expect("anchor entity expect a state");
            let anchor_orientation = orientations.get(anchor.anchor)
                .expect("for now anchor anchor expect orientation").0;

            let angle = anchor_orientation + anchor.angle;

            entity_state.pos[0] = anchor_state.pos[0] + anchor.distance * angle.cos();
            entity_state.pos[1] = anchor_state.pos[1] + anchor.distance * angle.sin();

            entity_state.vel = [0., 0.];
            entity_state.acc = [0., 0.];
        }
    }
}

pub struct SpringSystem;
impl specs::System<app::UpdateContext> for SpringSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        let (states, mut springs, entities) = arg.fetch(|world| {
            (
                world.read::<PhysicState>(),
                world.write::<PhysicSpring>(),
                world.entities(),
            )
        });
        for (mut spring, state, entity) in (&mut springs, &states, &entities).iter() {
            if let Some(anchor_state) = states.get(spring.anchor) {
                let self_to_anchor = sub(anchor_state.pos, state.pos);
                spring.angle = angle(self_to_anchor);
                spring.delta_len = norm(self_to_anchor) - spring.free_len;
            } else {
                spring.delta_len = 0.;
            }
        }
    }
}

pub struct PhysicSystem;
impl specs::System<app::UpdateContext> for PhysicSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        use std::f32::consts::PI;
        use std::f32;

        let (dynamics, mut states, dampings, forces, springs, types, mut physic_world, entities) = arg.fetch(|world| {
            (
                world.read::<PhysicDynamic>(),
                world.write::<PhysicState>(),
                world.read::<PhysicDamping>(),
                world.read::<PhysicForce>(),
                world.read::<PhysicSpring>(),
                world.read::<PhysicType>(),
                world.write_resource::<PhysicWorld>(),
                world.entities(),
            )
        });

        let dt = context.dt;

        let mut resolutions = FnvHashMap::<specs::Entity,Resolution>::default();

        for (_, state, typ, entity) in (&dynamics, &mut states, &types, &entities).iter() {
            let mut f = [0., 0.];

            if let Some(&PhysicDamping(damping)) = dampings.get(entity) {
                f[0] -= damping*state.vel[0];
                f[1] -= damping*state.vel[1];
            }
            if let Some(force) = forces.get(entity) {
                f[0] += force.coef*force.strength*force.angle.cos();
                f[1] += force.coef*force.strength*force.angle.sin();
            }
            if let Some(spring) = springs.get(entity) {
                f[0] += spring.coef*spring.delta_len*spring.angle.cos();
                f[1] += spring.coef*spring.delta_len*spring.angle.sin();
            }

            state.acc[0] = f[0]/typ.weight;
            state.acc[1] = f[1]/typ.weight;

            state.vel[0] += dt*state.acc[0];
            state.vel[1] += dt*state.acc[1];

            state.pos[0] += dt*state.vel[0];
            state.pos[1] += dt*state.vel[1];

            if typ.mask == 0 { continue }

            let shape_cast = ShapeCast {
                pos: state.pos,
                shape: typ.shape.clone(),
                mask: typ.mask,
                group: typ.group,
                not: vec!(entity),
            };

            physic_world.apply_on_shape(&shape_cast, &mut |other_info,collision| {
                let other_type = types.get(other_info.entity).expect("physic entity expect type component");
                let rate = match (typ.weight, other_type.weight) {
                    (f32::MAX, f32::MAX) => 0.5,
                    (f32::MAX, _) => 1.,
                    (_, f32::MAX) => 0.,
                    _ => typ.weight/(typ.weight+other_type.weight),
                };

                if rate != 1. {
                    let resolution = Resolution {
                        dx: collision.delta_x*(1.-rate),
                        dy: collision.delta_y*(1.-rate),
                    };
                    resolutions.entry(entity).or_insert(Resolution::none()).push(resolution);
                }
                if rate != 0. {
                    let resolution = Resolution {
                        dx: -collision.delta_x*rate,
                        dy: -collision.delta_y*rate,
                    };
                    resolutions.entry(other_info.entity).or_insert(Resolution::none()).push(resolution);
                }
            });

            physic_world.insert_dynamic(EntityInformation {
                entity: entity,
                pos: state.pos,
                group: typ.group,
                shape: typ.shape.clone(),
                mask: typ.mask,
            });
        }

        for (entity,res) in resolutions {
            let state = states.get_mut(entity).unwrap();
            let typ = types.get(entity).unwrap();

            state.pos[0] += res.dx;
            state.pos[1] += res.dy;

            match typ.collision {
                CollisionBehavior::Bounce => {
                    let angle = state.vel[1].atan2(state.vel[0]) + PI;
                    state.vel[0] = angle.cos();
                    state.vel[1] = angle.sin();
                },
                CollisionBehavior::Stop => state.vel = [0.,0.],
                CollisionBehavior::Back => {
                    state.vel[0] = -state.vel[0];
                    state.vel[1] = -state.vel[1];
                },
                CollisionBehavior::Persist => (),
            }
        }

        physic_world.movable = FnvHashMap::default(); // TODO only rewrite those that have been resolved
        for (_,state,typ,entity) in (&dynamics, &mut states, &types, &entities).iter() {
            physic_world.insert_dynamic(EntityInformation {
                entity: entity,
                pos: state.pos,
                group: typ.group,
                shape: typ.shape.clone(),
                mask: typ.mask,
            });
        }
    }
}
