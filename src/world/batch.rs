use super::spatial_hashing::{
    SpatialHashing,
    Location,
    Identifiable,
};
use super::BodyTrait;
use util::grid_raycast;
use std::ops::Rem;
use std::rc::Rc;
use std::collections::HashSet;
use std::cmp::Ordering;

pub struct Batch {
    pub unit: f64,
    static_hashmap: SpatialHashing<Rc<BodyTrait>>,
    dynamic_hashmap: SpatialHashing<Rc<BodyTrait>>,
}

impl Identifiable for Rc<BodyTrait> {
    fn id(&self) -> usize {
        (**self).id()
    }
}

impl Batch {
    pub fn new(unit: f64) -> Batch {
        Batch {
            unit: unit,
            static_hashmap: SpatialHashing::new(unit),
            dynamic_hashmap: SpatialHashing::new(unit),
        }
    }

    pub fn insert_static(&mut self, body: &Rc<BodyTrait>) {
        self.static_hashmap.insert(&body.location(),body);
    }

    pub fn insert_dynamic(&mut self, body: &Rc<BodyTrait>) {
        self.dynamic_hashmap.insert(&body.location(),body);
    }

    pub fn apply_locally<F: FnMut(&Rc<BodyTrait>)>(&self, loc: &Location, callback: &mut F) {
        self.static_hashmap.apply_locally(loc,callback);
        self.dynamic_hashmap.apply_locally(loc,callback);
    }

    pub fn apply_on_index<F: FnMut(&Rc<BodyTrait>)>(&self, index: &[i32;2], callback: &mut F) {
        self.static_hashmap.apply_on_index(index,callback);
        self.dynamic_hashmap.apply_on_index(index,callback);
    }

    /// callback return true when stop
    pub fn raycast<F: FnMut(&BodyTrait, f64, f64) -> bool>(&self, x: f64, y: f64, angle: f64, length: f64, callback: &mut F) {
        use std::f64::consts::PI;

        //println!("");
        //println!("raycast");

        let angle = angle.rem(PI);

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

        let mut bodies: Vec<(Rc<BodyTrait>,f64,f64)>;
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
            while let Some(body) = res.pop() {
                if !visited.contains(&body.id()) {
                    let intersections = body.raycast(a,b,c);
                    if let Some((x_min,y_min,x_max,y_max)) = intersections {
                        //println!("intersection");
                        //println!("start:{},end:{},min:{},max:{}",segment_start,segment_end,x_min,x_max);

                        if angle.abs() > PI/2. {
                            if segment_start <= x_max && x_min <= segment_end {
                                visited.insert(body.id());
                                //println!("intersection in segment");
                                let max = ((x0-x_min).powi(2) + (y0-y_min).powi(2)).sqrt();
                                let mut min = ((x0-x_max).powi(2) + (y0-y_max).powi(2)).sqrt();
                                if x_max > segment_end {
                                    min = -min;
                                }
                                bodies.push((body,min,max));
                            }
                        } else {
                            if segment_start <= x_max && x_min <= segment_end {
                                visited.insert(body.id());
                                //println!("intersection in segment");
                                let mut min = ((x0-x_min).powi(2) + (y0-y_min).powi(2)).sqrt();
                                let max = ((x0-x_max).powi(2) + (y0-y_max).powi(2)).sqrt();
                                if x_min < segment_start {
                                    min = -min;
                                }
                                bodies.push((body,min,max));
                            }
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

            for (body,min,max) in bodies {
                let body = &*body;
                visited.insert(body.id());
                if callback(body,min,max) {
                    return;
                }
            }
        }
    }

    pub fn get_on_segment(&self) {
    }

    pub fn get_on_index(&self, index: &[i32;2]) -> Vec<Rc<BodyTrait>> {
        let mut vec = Vec::new();
        vec.append(&mut self.static_hashmap.get_on_index(index));
        vec.append(&mut self.dynamic_hashmap.get_on_index(index));
        vec
    }

    pub fn get_locally(&self, loc: &Location) -> Vec<Rc<BodyTrait>> {
        let mut vec = Vec::new();
        vec.append(&mut self.static_hashmap.get_locally(loc));
        vec.append(&mut self.dynamic_hashmap.get_locally(loc));
        vec
    }

    pub fn clear_dynamic(&mut self) {
        self.dynamic_hashmap.clear();
    }
}

