use super::spatial_hashing::{
    SpatialHashing,
    Location,
};

pub struct Batch<'l,T: 'l+Clone> {
    static_hashmap: &'l SpatialHashing<T>,
    dynamic_hashmap: &'l SpatialHashing<T>,
}

impl<'l, T: Clone> Batch<'l, T> {
    pub fn new(static_hashmap: &'l SpatialHashing<T>, dynamic_hashmap: &'l SpatialHashing<T>) -> Batch<'l, T> {
        Batch {
            static_hashmap: static_hashmap,
            dynamic_hashmap: dynamic_hashmap,
        }
    }

    pub fn apply_locally<F: FnMut(&T)>(&self, loc: &Location, callback: &mut F) {
        self.static_hashmap.apply_locally(loc,callback);
        self.dynamic_hashmap.apply_locally(loc,callback);
    }

    pub fn apply_on_index<F: FnMut(&T)>(&self, index: &[i32;2], callback: &mut F) {
        self.static_hashmap.apply_on_index(index,callback);
        self.dynamic_hashmap.apply_on_index(index,callback);
    }

    pub fn get_on_index(&self, index: &[i32;2]) -> Vec<T> {
        let mut vec = Vec::new();
        vec.append(&mut self.static_hashmap.get_on_index(index));
        vec.append(&mut self.dynamic_hashmap.get_on_index(index));
        vec
    }

    pub fn get_locally(&self, loc: &Location) -> Vec<T> {
        let mut vec = Vec::new();
        vec.append(&mut self.static_hashmap.get_locally(loc));
        vec.append(&mut self.dynamic_hashmap.get_locally(loc));
        vec
    }
}

