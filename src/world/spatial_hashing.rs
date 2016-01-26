use std::collections::{
    HashMap,
    HashSet,
};

pub struct Location {
    pub up: f64,
    pub down: f64,
    pub left: f64,
    pub right: f64,
}

pub trait Identifiable {
    fn id(&self) -> usize;
}

pub struct SpatialHashing<T: Clone+Identifiable> {
    unit: f64,
    hashmap: HashMap<[i32;2],Vec<T>>,
}

impl<T: Clone+Identifiable> SpatialHashing<T> {
    pub fn new(unit: f64) -> SpatialHashing<T> {
        SpatialHashing {
            unit: unit,
            hashmap: HashMap::new(),
        }
    }

    fn index(&self, loc: &Location) -> Vec<[i32;2]> {

        let min_x = (loc.left/self.unit) as i32;
        let max_x = (loc.right/self.unit) as i32;
        let min_y = (loc.down/self.unit) as i32;
        let max_y = (loc.up/self.unit) as i32;

        let mut vec = Vec::new();
        for x in min_x..max_x+1 {
            for y in min_y..max_y+1 {
                vec.push([x,y]);
            }
        }

        vec
    }

    pub fn insert(&mut self, loc: &Location, obj: &T) {
        let index = self.index(loc);

        for i in &index {
            if let Some(vec) = self.hashmap.get_mut(i) {
                vec.push(obj.clone());
                continue;
            }
            self.hashmap.insert(*i,vec![obj.clone()]);
        }
    }

    pub fn apply_locally<F: FnMut(&T)>(&self, loc: &Location, callback: &mut F) {
        let index = self.index(loc);
        let mut visited = HashSet::<usize>::new();
        for i in &index {
            self.apply_on_index(i,&mut |t: &T| {
                if !visited.contains(&t.id()) {
                    callback(t);
                    visited.insert(t.id());
                }
            });
        }
    }

    pub fn apply_on_index<F: FnMut(&T)>(&self, index: &[i32;2], callback: &mut F) {
        if let Some(vec) = self.hashmap.get(index) {
            for t in vec {
                callback(t);
            }
        }
    }

    pub fn get_on_index(&self, index: &[i32;2]) -> Vec<T> {
        if let Some(vec) = self.hashmap.get(index) {
            vec.clone()
        } else {
            Vec::new()
        }
    }

    pub fn get_locally(&self, loc: &Location) -> Vec<T> {
        let index = self.index(loc);
        let mut vec = Vec::new();
        let mut got = HashSet::<usize>::new();
        for i in &index {
            let mut new_vec = self.get_on_index(i);
            while let Some(t) = new_vec.pop() {
                if !got.contains(&t.id()) {
                    got.insert(t.id());
                    vec.push(t);
                }
            }
        }
        vec
    }

    pub fn clear(&mut self) {
        self.hashmap.clear();
    }

    pub fn unit(&self) -> f64 {
        self.unit
    }
}
