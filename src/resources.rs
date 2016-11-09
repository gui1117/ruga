use physics::{self, EntityInformation, ContinueOrStop, Collision, RayCast, ShapeCast};
use components::*;
use specs::Join;
use std::collections::HashSet;

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
        use ::std::f32::consts::PI;
        use ::std::cmp::Ordering;

        let angle = ::utils::minus_pi_pi(ray.angle);
        let x0 = ray.origin[0];
        let y0 = ray.origin[1];
        let x1 = x0 + ray.length*angle.cos();
        let y1 = y0 + ray.length*angle.sin();
        let cells = physics::grid_raycast(x0, y0, x1, y1);
        let ray_min_x = x0.min(x1);
        let ray_max_x = x0.max(x1);

        // equation ax + by + c = 0
        let equation = if angle.abs() == PI || angle == 0. {
            (0.,1.,-y0)
        } else {
            let b = -1./angle.tan();
            (1.,b,-x0-b*y0)
        };

        let mut visited = HashSet::new();

        for cell in cells {
            // abscisse of start and end the segment of
            // the line that is in the current square

            let segment_min_x = (cell[0] as f32).max(ray_min_x);
            let segment_max_x = ((cell[0]+1) as f32).min(ray_max_x);

            let null_vec = Vec::new();
            let mut bodies = Vec::new();

            let entities = self.movable.get(&cell).unwrap_or(&null_vec).iter()
                .chain(self.inert.get(&cell).unwrap_or(&null_vec).iter());

            for entity in entities {
                if entity.group & ray.mask == 0 { continue }
                if entity.mask & ray.group == 0 { continue }
                if visited.contains(&entity.entity) { continue }

                if let Some((x_min,y_min,x_max,y_max)) = entity.shape.raycast(entity.pos, equation) {
                    if segment_start > x_max || x_min > segment_end { continue }

                    // TODO set min max
                    let min = 0f32;
                    let max = 0f32;
                    visited.insert(entity.entity);
                    bodies.push((entity,min,max));
                }
            }

            bodies.sort_by(|&(_,min_a,_),&(_,min_b,_)| {
                if min_a > min_b { Ordering::Greater }
                else if min_a == min_b { Ordering::Equal }
                else { Ordering::Less }
            });

            for b in bodies {
                if let ContinueOrStop::Stop = callback(b) {
                    return;
                }
            }
        }
    }
}
