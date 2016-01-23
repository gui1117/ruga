use super::spatial_hashing::Spatialable;

pub trait Localisable {
    fn up (&self) -> f64;
    fn down (&self) -> f64;
    fn left (&self) -> f64;
    fn right (&self) -> f64;
}

pub struct Localisation {
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
}

impl Localisable for Localisation {
    fn up(&self) -> f64 {
        self.y + self.height/2.
    }
    fn down(&self) -> f64 {
        self.y - self.height/2.
    }
    fn left(&self) -> f64 {
        self.x - self.width/2.
    }
    fn right(&self) -> f64 {
        self.x + self.width/2.
    }
}

impl Spatialable for Localisation {
    fn spatial(&self) -> (f64,f64,f64,f64) {
        (self.left(), self.right(), self.down(), self.up())
    }
}
