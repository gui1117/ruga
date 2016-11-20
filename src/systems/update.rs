use app::UpdateContext;
use fnv::FnvHashMap;
use app;
use specs;
use physics::{EntityInformation, Resolution, ShapeCast, Collision, CollisionBehavior};
use components::*;
use resources::*;

pub fn add_systems(planner: &mut ::specs::Planner<UpdateContext>) {
    planner.add_system(PhysicSystem, "physic", 0);
}

pub struct PhysicSystem;
impl specs::System<app::UpdateContext> for PhysicSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        use std::f32::consts::PI;
        use std::f32;
        use specs::Join;

        let (dynamics, mut states, forces, types, mut physic_world, entities) = arg.fetch(|world| {
            (
                world.read::<PhysicDynamic>(),
                world.write::<PhysicState>(),
                world.read::<PhysicForce>(),
                world.read::<PhysicType>(),
                world.write_resource::<PhysicWorld>(),
                world.entities(),
            )
        });

        let dt = context.dt;

        let mut resolutions = FnvHashMap::<specs::Entity,Resolution>::default();

        physic_world.movable = FnvHashMap::default();
        for (_, state, force, typ, entity) in (&dynamics, &mut states, &forces, &types, &entities).iter() {
            state.acc[0] = (typ.force*force.strength*force.angle.cos() - typ.damping*state.vel[0])/typ.weight;

            state.acc[1] = (typ.force*force.strength*force.angle.sin() - typ.damping*state.vel[1])/typ.weight;

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
            };

            physic_world.apply_on_shape(&shape_cast, &mut |other_info,collision| {
                let other_type = types.get(other_info.entity).expect("physic entity expect type component");
                let rate = match (typ.weight, other_type.weight) {
                    (f32::MAX, f32::MAX) => 0.5,
                    (f32::MAX, _) => 1.,
                    (_, f32::MAX) => 0.,
                    (0., 0.) => 0.5,
                    (0., _) => 0.,
                    (_, 0.) => 1.,
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
                    resolutions.entry(entity).or_insert(Resolution::none()).push(resolution);
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
