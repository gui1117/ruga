use std::ops::Add;

/// simple structure, it represents a coordinate
#[derive(Clone)]
pub struct Point {
	pub x: f64,
	pub y: f64,
}

impl Point {
	/// return the angle between the vector ba and bc.
	/// The angle is in ]-Pi,Pi]
	pub fn angle(a: &Point, b: &Point, c: &Point) -> f64 {
		let u = Point { x: a.x-b.x, y: a.y-b.y };
		let v = Point { x: c.x-b.x, y: c.y-b.y };
		let vectorial_product = u.x*v.y-u.y*v.x;
		let scalar_product = u.x*v.x+u.y*v.y;
		vectorial_product.atan2(scalar_product)
	}
	
	/// return the angle between 0x and the point ad 
	/// a vector
	pub fn angle_0x(&self) -> f64 {
		Self::angle(
			&Point { x: 1., y: 0. },
			&Point { x: 0., y: 0. },
			self)
	}
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
