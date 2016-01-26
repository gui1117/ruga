use super::spatial_hashing::{
    SpatialHashing,
    Location,
    Identifiable,
};
use super::BodyTrait;
use std::rc::Rc;

pub struct Batch {
    unit: f64,
    static_hashmap: SpatialHashing<Rc<BodyTrait>>,
    dynamic_hashmap: SpatialHashing<Rc<BodyTrait>>,
}

impl Identifiable for Rc<BodyTrait> {
    fn id(&self) -> usize {
        self.id()
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
        //let unit = self.static_hashmap.unit();
        //let x0 = x;
        //let y0 = y;
        //let x1 = x+length*angle.cos();
        //let y1 = y+length*angle.sin();
        //let index_vec = grid_raycast(x0/unit, y0/unit, x1/unit, y1/unit);

        //// equation y = ax + b (we consider x0 and x1 never alined)
        //let a = (y1 - y0)/(x1 - x0);
        //let b = y0 -a*x0;

        //let mut bodies: Vec<(Rc<BodyTrait>,f64,f64)>;
        //let mut visited = HashSet::new();
        //for i in &index_vec {
        //    let segment_start = (i[0] as f64)*unit;
        //    let segment_end = ((i[0]+1) as f64)*unit;
        //    bodies = Vec::new();

        //    let mut res = self.static_hashmap.get_on_index(i);
        //    res.append(&mut self.dynamic_hashmap.get_on_index(i));
        //    while let Some(body) = res.pop() {
        //        if !visited.contains(&body.id()) {
        //            let op = body.raycast(a,b);
        //            if let Some((x_min,y_min,x_max,y_max)) = op {
        //                if segment_start < x_min && x_min < segment_end {
        //                    let min = ((x0-x_min).exp2() + (y0-y_min).exp2()).sqrt();
        //                    let max = ((x0-x_max).exp2() + (y0-y_max).exp2()).sqrt();
        //                    bodies.push((body,min,max));
        //                }
        //            }
        //        }
        //    }

        //    bodies.sort_by(|&(_,min_a,_),&(_,min_b,_)| {
        //        if min_a > min_b {
        //            Ordering::Less
        //        } else if min_a == min_b {
        //            Ordering::Equal
        //        } else {
        //            Ordering::Greater
        //        }
        //    });

        //    for (body,min,max) in bodies {
        //        let body = &*body;
        //        visited.insert(body.id());
        //        if callback(body,min,max) {
        //            return;
        //        }
        //    }
        //}
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

