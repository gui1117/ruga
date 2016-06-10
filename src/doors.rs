use specs;
use physic;
use app;

pub struct Door {
    signal: specs::Entity,
}
impl specs::Component for Door {
    type Storage = specs::VecStorage<Self>;
}

#[derive(Clone,Default)]
pub struct Sensor;
impl specs::Component for Sensor {
    type Storage = specs::NullStorage<Self>;
}

pub struct Signal {
    state: bool
}
impl specs::Component for Signal {
    type Storage = specs::VecStorage<Self>;
}

#[derive(Clone,Default)]
pub struct Multiplexer;
impl specs::Component for Multiplexer {
    type Storage = specs::NullStorage<Self>;
}

pub struct DoorSystem;
impl specs::System<app::UpdateContext> for DoorSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        use specs::Join;
        let (doors, mut statics, signals, states, types, mut physic_worlds, entities) = arg.fetch(|world| {
            (
                world.read::<Door>(),
                world.write::<physic::PhysicStatic>(),
                world.read::<Signal>(),
                world.write::<physic::PhysicState>(),
                world.read::<physic::PhysicType>(),
                world.write::<physic::PhysicWorld>(),
                world.entities(),
            )
        });

        let physic_world = physic_worlds.get_mut(context.master_entity)
            .expect("master entity expect physic_world component");

        for (door, entity) in (&doors, &entities).iter() {
            let closed = statics.get(entity).is_some();
            let signal = signals.get(door.signal).expect("door entity expect signal component");

            if signal.state != closed {
                let state = states.get(entity).expect("door entity expect state component");
                let typ = types.get(entity).expect("door entity expect typ component");

                if signal.state {
                    // open the door
                    physic_world.remove_static(entity,&state.position,&typ.shape);
                    statics.remove(entity);
                } else {
                    // close the door
                    physic_world.insert_static(entity,&state.position,&typ.shape);
                    statics.insert(entity,physic::PhysicStatic);
                }
            }
        }
    }
}

pub struct SensorSystem;
impl specs::System<app::UpdateContext> for SensorSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        use specs::Join;
        let (sensors, mut signals, states, types, physic_worlds, entities) = arg.fetch(|world| {
            (
                world.read::<Sensor>(),
                world.write::<Signal>(),
                world.read::<physic::PhysicState>(),
                world.read::<physic::PhysicType>(),
                world.read::<physic::PhysicWorld>(),
                world.entities(),
            )
        });

        let physic_world = physic_worlds.get(context.master_entity)
            .expect("master entity expect physic_world component");

        for (_, entity) in (&sensors, &entities).iter() {
            let signal = signals.get_mut(entity).expect("door entity expect signal component");
            let state = states.get(entity).expect("door entity expect state component");
            let typ = types.get(entity).expect("door entity expect typ component");

            //TODO
            let signal = true;//character_on_top;
        }
    }
}

