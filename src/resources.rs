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
    static_hashmap: ::fnv::FnvHashMap<[i32;2],Vec<::physics::EntityInformation>>,
    movable_hashmap: ::fnv::FnvHashMap<[i32;2],Vec<::physics::EntityInformation>>,
}
impl PhysicWorld {
    pub fn new() -> Self {
        PhysicWorld {
            static_hashmap: ::fnv::FnvHashMap::default(),
            movable_hashmap: ::fnv::FnvHashMap::default(),
        }
    }
}
