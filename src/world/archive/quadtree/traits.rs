/// trait required by objects inserted in the quadtree,
/// used when returning the possible collision of an object
pub trait Identifiable {
    fn id(&self) -> usize;
}

/// trait required by objects inserted in the quadtree,
/// used when inserting an object in the quadtree,
/// an object is consider up to a certain y when its 
/// bounding box is **entirely** up to this y.
/// Same for down, left and right...
pub trait Localisable {
    fn up (&self, f64) -> bool;
    fn down (&self, f64) -> bool;
    fn left (&self, f64) -> bool;
    fn right (&self, f64) -> bool;
}
