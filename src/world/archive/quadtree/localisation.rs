use super::Localisable;

pub struct Localisation {
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
}

impl Localisable for Localisation {
    fn up(&self, y: f64) -> bool {
        self.y - self.height/2. > y
    }
    fn down(&self, y: f64) -> bool {
        self.y + self.height/2. < y
    }
    fn left(&self, x: f64) -> bool {
        self.x - self.width/2. > x
    }
    fn right(&self, x: f64) -> bool {
        self.y + self.width/2. < x
    }
}
