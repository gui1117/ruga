use spatial_hashing::{SpatialHashing, Identifiable};
use Entity;
use EntityManagement;
use body::{PhysicType, Flags, Location};
use utils::grid_raycast;
use utils::minus_pi_pi;

use std::rc::Rc;
use std::collections::HashSet;
use std::cmp::Ordering;

pub struct World<M: EntityManagement> {
    pub unit: f64,
    pub time: f64,

    entities: Vec<Rc<Entity<M>>>,
    static_hashmap: SpatialHashing<Rc<Entity<M>>>,
    dynamic_hashmap: SpatialHashing<Rc<Entity<M>>>,
}

impl<M: EntityManagement> Identifiable for Rc<Entity<M>> {
    fn id(&self) -> usize {
        self.body().id
    }
}

impl<M: EntityManagement> World<M> {
    pub fn new(unit: f64) -> World<M> {
        World {
            unit: unit,
            time: 0.,
            entities: Vec::new(),
            static_hashmap: SpatialHashing::new(unit),
            dynamic_hashmap: SpatialHashing::new(unit),
        }
    }

    pub fn unit(&self) -> f64 {
        self.unit
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn render(&mut self, frame_manager: &mut M::FrameManager) {
        for entity in &self.entities {
            entity.render(frame_manager);
        }
    }

    pub fn update(&mut self, dt: f64, effect_manager: &mut M::EffectManager) {
        for entity in &self.entities {
            entity.update(dt,&self,effect_manager);
        }

        let mut i = 0;
        while i < self.entities.len() {
            let b = self.entities[i].body().life <= 0.;
            if b {
                self.entities.swap_remove(i);
            } else {
                i += 1;
            }
        }

        self.clear_dynamic();
        for entity in &self.entities {
            {
                let location = entity.body().location();
                let mask = entity.body().mask;
                let mut callback = |other: &Entity<M>| {
                    if entity.body().collide(other.body()) {
                        entity.mut_body().resolve_collision(other.body());
                        other.mut_body().resolve_collision(entity.body());
                        entity.on_collision(other.mut_body());
                        other.on_collision(entity.mut_body());
                    }
                };
                self.apply_locally(mask,&location,&mut callback);
            }
            self.dynamic_hashmap.insert_locally(&entity.body().location(),entity);
        }
    }

    pub fn entities(&self) -> &Vec<Rc<Entity<M>>> {
        &self.entities
    }

    pub fn insert(&mut self, entity: &Rc<Entity<M>>) {
        match entity.body().physic_type {
            PhysicType::Static => self.static_hashmap.insert_locally(&entity.body().location(),entity),
            _ => self.dynamic_hashmap.insert_locally(&entity.body().location(),entity),
        }
    }

    pub fn apply_on_group<F: FnMut(&Entity<M>)>(&self, mask: Flags, callback: &mut F) {
        for entity in &self.entities {
            if entity.body().mask & mask != 0 {
                callback(&**entity);
            }
        }
    }

    pub fn apply_locally<F: FnMut(&Entity<M>)>(&self, mask: Flags, loc: &Location, callback: &mut F) {
        self.static_hashmap.apply_locally(loc, &mut |entity: &Rc<Entity<M>>| {
            if (entity.body().mask & mask != 0) && entity.body().in_location(loc) {
                callback(&**entity);
            }
        });
        self.dynamic_hashmap.apply_locally(loc, &mut |entity: &Rc<Entity<M>>| {
            if (entity.body().mask & mask != 0) && entity.body().in_location(loc) {
                callback(&**entity);
            }
        });
    }

    pub fn apply_on_index<F: FnMut(&Entity<M>)>(&self, mask: Flags, index: &[i32;2], callback: &mut F) {
        let c = &mut |entity: &Rc<Entity<M>>| {
            if entity.body().group & mask != 0 {
                callback(&**entity);
            }
        };
        self.static_hashmap.apply_on_index(index,c);
        self.dynamic_hashmap.apply_on_index(index,c);
    }

    /// callback return true when stop
    pub fn raycast<F: FnMut(&Entity<M>, f64, f64) -> bool>(&self, mask: Flags, x: f64, y: f64, angle: f64, length: f64, callback: &mut F) {
        use std::f64::consts::PI;

        //println!("");
        //println!("raycast");

        let angle = minus_pi_pi(angle);

        let unit = self.static_hashmap.unit();
        let x0 = x;
        let y0 = y;
        let x1 = x+length*angle.cos();
        let y1 = y+length*angle.sin();
        let index_vec = grid_raycast(x0/unit, y0/unit, x1/unit, y1/unit);

        // equation ax + by + c = 0
        let (a,b,c) = if angle.abs() == PI || angle == 0. {
            (0.,1.,-y)
        } else {
            let b = -1./angle.tan();
            (1.,b,-x-b*y)
        };

        let line_start = x0.min(x1);
        let line_end = x0.max(x1);

        let mut bodies: Vec<(Rc<Entity<M>>,f64,f64)>;
        let mut visited = HashSet::new();
        for i in &index_vec {
            //println!("index:{:?}",i);
            // abscisse of start and end the segment of
            // the line that is in the current square
            let segment_start = ((i[0] as f64)*unit).max(line_start);
            let segment_end = (((i[0]+1) as f64)*unit).min(line_end);

            bodies = Vec::new();

            let mut res = self.static_hashmap.get_on_index(i);
            res.append(&mut self.dynamic_hashmap.get_on_index(i));
            res.retain(|entity| {
                (entity.body().mask & mask != 0) && !visited.contains(&entity.body().id)
            });
            while let Some(entity) = res.pop() {
                let intersections = entity.body().raycast(a,b,c);
                if let Some((x_min,y_min,x_max,y_max)) = intersections {
                    //println!("intersection");
                    //println!("start:{},end:{},min:{},max:{}",segment_start,segment_end,x_min,x_max);

                    if angle.abs() > PI/2. {
                        if segment_start <= x_max && x_min <= segment_end {
                            visited.insert(entity.body().id);
                            //println!("intersection in segment");
                            let max = ((x0-x_min).powi(2) + (y0-y_min).powi(2)).sqrt();
                            let mut min = ((x0-x_max).powi(2) + (y0-y_max).powi(2)).sqrt();
                            if x_max > segment_end {
                                min = -min;
                            }
                            bodies.push((entity,min,max));
                        }
                    } else {
                        if segment_start <= x_max && x_min <= segment_end {
                            visited.insert(entity.body().id);
                            //println!("intersection in segment");
                            let mut min = ((x0-x_min).powi(2) + (y0-y_min).powi(2)).sqrt();
                            let max = ((x0-x_max).powi(2) + (y0-y_max).powi(2)).sqrt();
                            if x_min < segment_start {
                                min = -min;
                            }
                            bodies.push((entity,min,max));
                        }
                    }
                }
            }

            bodies.sort_by(|&(_,min_a,_),&(_,min_b,_)| {
                if min_a > min_b {
                    Ordering::Greater
                } else if min_a == min_b {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            });

            for (entity,min,max) in bodies {
                let entity = &*entity;
                visited.insert(entity.body().id);
                if callback(&*entity,min,max) {
                    return;
                }
            }
        }
    }

    pub fn get_on_segment<F: FnMut(&mut Entity<M>, f64, f64) -> bool>(&self, _mask: Flags, _x: f64, _y: f64, _angle: f64, _length: f64, _callback: &mut F) {
        assert!(false);
    }

    pub fn get_on_index(&self, mask: Flags, index: &[i32;2]) -> Vec<Rc<Entity<M>>> {
        let mut vec = Vec::new();
        vec.append(&mut self.static_hashmap.get_on_index(index));
        vec.append(&mut self.dynamic_hashmap.get_on_index(index));
        vec.retain(&mut |entity: &Rc<Entity<M>>| {
            entity.body().mask & mask != 0
        });
        vec
    }

    pub fn get_locally(&self, mask: Flags, loc: &Location) -> Vec<Rc<Entity<M>>> {
        let mut vec = Vec::new();
        vec.append(&mut self.static_hashmap.get_locally(loc));
        vec.append(&mut self.dynamic_hashmap.get_locally(loc));
        vec.retain(&mut |entity: &Rc<Entity<M>>| {
            let entity = entity.body();
            (entity.mask & mask != 0) && (entity.in_location(loc))
        });
        vec
    }

    pub fn get_on_group(&self, mask: Flags) -> Vec<Rc<Entity<M>>> {
        let mut vec = Vec::new();
        for entity in &self.entities {
            if entity.body().mask & mask != 0 {
                vec.push(entity.clone());
            }
        }
        vec
    }

    pub fn clear_dynamic(&mut self) {
        self.dynamic_hashmap.clear();
    }
}

