use physics::{EntityInformation, ContinueOrStop, Collision, RayCast, ShapeCast};
use components::*;
use specs::Join;

macro_rules! impl_resource {
    ($($typ:ident,)*) => { impl_resource!{ $($typ),* } };
    ($($typ:ident),*) => {
        pub fn add_resource(world: &mut ::specs::World) {
            $(world.add_resource(::resources::$typ::new());)*
        }
    };
}

impl_resource!{
    Notifications,
    PhysicWorld,
}

pub struct Notifications(pub Vec<(String, usize)>);
impl Notifications {
    pub fn new() -> Self {
        Notifications(Vec::new())
    }
}

pub struct PhysicWorld {
    inert: ::fnv::FnvHashMap<[i32;2],Vec<EntityInformation>>,
    movable: ::fnv::FnvHashMap<[i32;2],Vec<EntityInformation>>,
}
impl PhysicWorld {
    pub fn new() -> Self {
        PhysicWorld {
            inert: ::fnv::FnvHashMap::default(),
            movable: ::fnv::FnvHashMap::default(),
        }
    }
    pub fn fill(&mut self, world: &::specs::World) {
        let dynamics = world.read::<PhysicDynamic>();
        let statics = world.read::<PhysicStatic>();
        let states = world.read::<PhysicState>();
        let types = world.read::<PhysicType>();
        let entities = world.entities();

        self.inert.clear();
        self.movable.clear();

        for (_,state,typ,entity) in (&dynamics, &states, &types, &entities).iter() {
            let info = EntityInformation {
                entity: entity,
                pos: state.pos,
                group: typ.group,
                mask: typ.mask,
                shape: typ.shape.clone(),
            };
            self.insert_dynamic(info);
        }
        for (_,state,typ,entity) in (&statics, &states, &types, &entities).iter() {
            let info = EntityInformation {
                entity: entity,
                pos: state.pos,
                group: typ.group,
                mask: typ.mask,
                shape: typ.shape.clone(),
            };
            self.insert_static(info);
        }
    }
    pub fn insert_dynamic(&mut self, info: EntityInformation) {
        for cell in info.shape.cells(info.pos) {
            self.movable.entry(cell).or_insert(Vec::new()).push(info.clone());
        }
    }
    pub fn insert_static(&mut self, info: EntityInformation) {
        for cell in info.shape.cells(info.pos) {
            self.inert.entry(cell).or_insert(Vec::new()).push(info.clone());
        }
    }
    pub fn apply_on_shape<F: FnMut(&EntityInformation, &Collision)>(&self, shape: &ShapeCast, callback: &mut F) {
        unimplemented!();
    }
    pub fn raycast<F: FnMut((&EntityInformation,f32,f32)) -> ContinueOrStop>(&self, ray: &RayCast, callback: &mut F) {
        unimplemented!();
    }
}
