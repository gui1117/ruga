//use world::quadtree::Localisable;
use super::shape::Shape;

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

	/// return whether the point is in the shape or not
	pub fn in_shape(&self, shape: &Shape,) -> bool {
		for n in &shape.normals {
			let (min,max) = shape.min_max(*n + shape.angle);
			
			let dir = Point { 
                x: (*n+shape.angle).cos(), 
                y: (*n+shape.angle).sin() 
            };

			let p = self.x*dir.x + self.y*dir.y;
			
			if p < min || p > max {
				return false;
			}
		}
		true
	}
}

//impl Localisable for Point {
//	fn up(&self, y: f64) -> bool {
//		self.y > y
//	}
//	fn down(&self, y: f64) -> bool {
//		self.y  < y
//	}
//	fn left(&self, x: f64) -> bool {
//		self.x < x
//	}
//	fn right(&self, x: f64) -> bool {
//		self.x > x
//	}
//}

/// simple structure, it represents a box.
/// the downleft point is the point at the maximum y 
/// and lowest x.
#[derive(Clone)]
pub struct Rectangle {
	pub downleft: Point,
	pub width: f64,
	pub height: f64,
}
