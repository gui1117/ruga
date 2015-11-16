use std::f64;
use std::f64::consts::PI;

/// simple structure, it represents a coordinate
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
	pub fn in_shape(&self, x: f64, y: f64, angle: f64, shape: &Shape,) -> bool {
		let (h_min,h_max) = shape.min_max(x, y, angle, 0.);
		if self.x < h_min || self.x > h_max {
			return false;
		}

		let (v_min,v_max) = shape.min_max(x, y, angle, PI/2.);
		if self.y < v_min || self.y > v_max {
			return false;
		}

		false
	}
}

pub struct Segment {
	origin: Point,
	length: f64,
	angle: f64,
}

impl Segment {
	/// return (whether the segment is intersecting the shape, the first intersection, the second
	/// intersection)
	pub fn intersect(&self, x: f64, y: f64, angle: f64, shape: &Shape) -> bool {
		let (dir_min,dir_max) = shape.min_max(x, y, angle, self.angle);
		let (nor_min,nor_max) = shape.min_max(x, y, angle, self.angle + PI/2.);

		if self.origin.x > dir_max || self.origin.x + self.length < dir_min {
			return false;
		}
		if self.origin.y < nor_min || self.origin.y > nor_max {
			return false;
		}

		true
	}
}

/// simple structure, it represents a box.
/// the downleft point is the point at the maximum y 
/// and lowest x.
pub struct Rectangle {
	pub downleft: Point,
	pub width: f64,
	pub height: f64,
}

/// Shape is a convex polygon.
/// It also contain a bounding box
pub struct Shape {
	pub edges: Vec<Point>,
	pub normals: Vec<f64>,
}

impl Shape {
	/// create a new Shape, it panics if the shape isn't a convex polygon.
	/// Also it cannot be line
	pub fn new(mut edges: Vec<Point>) -> Shape {
		{
			let mut i = 0;
			while edges.len() > 3 && i < edges.len() {
				let len = edges.len();
				match Point::angle(&edges[i%len], &edges[(i+1)%len], &edges[(i+2)%len]) {
					0. => { edges.remove((i+1)%len); },
					_ => { i += 1; },

				}
			}
			if edges.len() < 3 {panic!("try to create a shape with less than 3 edges not aligned")}
		}

		{
			let witness = Point::angle(&edges[0],&edges[1],&edges[2]) > 0.;
			for i in 1..edges.len()-1 {
				let len = edges.len();
				if (Point::angle(&edges[i],&edges[(i+1)%len],&edges[(i+2)%len]) > 0.) != witness {
					panic!("try to create a not convex shape");
				}
			}
		}

		let v = Point { x: edges[0].x-edges[edges.len()-1].x, y: edges[0].y-edges[edges.len()-1].y };
		let mut normals = vec![v.angle_0x()-f64::consts::PI/2.];
		for i in 0..edges.len()-1 {
			let j = i+1;
			let v = Point { x: edges[j].x-edges[i].x, y: edges[j].y-edges[i].y };
			normals.push(v.angle_0x()-f64::consts::PI/2.);
		}

		Shape { 
			edges: edges,
			normals: normals,
		}
	}

	/// to have the minimal and maximal projection on the axe of
	/// an angle, also the shape is considered relative to
	/// the point x and y
	pub fn min_max(&self, x: f64, y: f64, shape_angle: f64, dir_angle: f64) -> (f64,f64) {
		let rel_dir = Point { x:f64::cos(dir_angle-shape_angle), y:f64::sin(dir_angle-shape_angle) };
		let mut min = f64::INFINITY;
		let mut max = f64::NEG_INFINITY;

		for edge in &self.edges {
			let cross = edge.x*rel_dir.x + edge.y*rel_dir.y;
			if cross > max { max = cross; }
			if cross < min { min = cross; }
		}

		let dir = Point { x:f64::cos(dir_angle), y:f64::sin(dir_angle) };
		let cross = x*dir.x + y*dir.y;
		(min+cross,max+cross)
	}

	/// compute if two shape each relative to a point x and y and also with an angle
	/// are overlapping or not
	/// it return a boolean for overlapping or not and the angle of minimal overlap and the length
	/// of overlapping 
	/// the vector returned is the vector you to move b so it doesn't overlap anymore
	pub fn overlap(a_x: f64, a_y: f64, a_angle: f64, a_shape: &Shape, b_x: f64, b_y: f64, b_angle:f64, b_shape: &Shape) -> (bool,f64,f64) {
		use std::f64::consts::PI;

		let mut min_angle = 0.;
		let mut min_length = f64::INFINITY;

		for n in &a_shape.normals {
			let (a_min,a_max) = a_shape.min_max(a_x, a_y, a_angle, *n + a_angle);
			let (b_min,b_max) = b_shape.min_max(b_x, b_y, b_angle, *n + a_angle);
			
			let length = (a_max-b_min).min(b_max-a_min);

			if length <= 0. {
				return (false,0.,0.);
			} else if min_length > length {
				min_length = length;
				
				if length == a_max-b_min {
					min_angle = *n + a_angle;
				} else {
					min_angle = *n + a_angle + PI;
				}
			}
		}

		for n in &b_shape.normals {
			let (a_min,a_max) = a_shape.min_max(a_x, a_y, a_angle, *n + b_angle);
			let (b_min,b_max) = b_shape.min_max(b_x, b_y, b_angle, *n + b_angle);

			let length = (a_max-b_min).min(b_max-a_min);

			if length <= 0. {
				return (false,0.,0.);
			} else if min_length > length {
				min_length = length;

				if length == a_max-b_min {
					min_angle = *n + b_angle;
				} else {
					min_angle = *n + b_angle + PI;
				}
			}
		}

		(true,min_length,min_angle)
	}

}

/*----------------------------------------*/
/*----------------------------------------*/
/*----------------------------------------*/

#[cfg(test)]
use std;

#[test]
fn point_angle() {
	use std::f64::consts::PI;

	let a = Point { x: 0., y: 0.};
	let b = Point { x: 1., y: 1.};
	let c = Point { x: 0., y: 1.};
	let pi = std::f64::consts::PI;
	assert_eq!(Point::angle(&a,&b,&c), -pi/4.);

	let a = Point { x: -10., y: -10.};
	let b = Point { x: 1., y: 1.};
	let c = Point { x: -1., y: 1.};
	let pi = std::f64::consts::PI;
	assert_eq!(Point::angle(&a,&b,&c), -pi/4.);

	let a = Point { x: 0., y: 0.};
	let b = Point { x: -1., y: 0.};
	let c = Point { x: -2., y: -1.};
	let pi = std::f64::consts::PI;
	assert_eq!(Point::angle(&a,&b,&c), -3./4.*pi);

	let a = Point { x: 0., y: 0.};
	let b = Point { x: -1., y: -1.};
	let c = Point { x: -1., y: -2.};
	let pi = std::f64::consts::PI;
	assert_eq!(Point::angle(&a,&b,&c), -3./4.*pi);

	let a = Point { x: 1., y: 1.};
	let b = Point { x: 1., y: 5.};
	let c = Point { x: 11., y: 5.};
	let pi = std::f64::consts::PI;
	assert_eq!(Point::angle(&a,&b,&c), pi/2.);

	let a = Point { x: 1., y: 1. };
	assert_eq!(a.angle_0x(),PI/4.);
}

#[test]
#[should_panic]
fn new_wrong_shape() {
	//one edge
	let p1 = Point{ x: 0.0, y: 0.0 };
	let _s = Shape::new(vec![p1]);
	
	//two edge
	let p1 = Point{ x: 0.0, y: 0.0 };
	let p2 = Point{ x: 0.0, y: 0.0 };
	let _s = Shape::new(vec![p1,p2]);

	//all edges aligned
	let p1 = Point{ x: 0., y: 0. };
	let p2 = Point{ x: 1., y: 1. };
	let p3 = Point{ x: 10., y: 10. };
	let p4 = Point{ x: -10., y: -10. };
	let p5 = Point{ x: 12., y: 12. };
	let p6 = Point{ x: 15., y: 15. };
	let _s = Shape::new(vec![p1,p2,p3,p4,p5,p6]);
}

#[test]
fn new_shape() {
	//standard
	let p1 = Point{ x: 0., y: 0. };
	let p2 = Point{ x: 1., y: 1. };
	let p3 = Point{ x: 0., y: 1. };
	let s = Shape::new(vec![p1,p2,p3]);

	assert_eq!(s.edges.len(), 3);
	assert_eq!(s.normals.len(), 3);
	assert_eq!(s.normals[0],-f64::consts::PI);
	assert_eq!(s.normals[1],-f64::consts::PI/4.);
	assert_eq!(s.normals[2],f64::consts::PI/2.);

	//with same edges
	let p0 = Point{ x: 0., y: 0. };
	let p1 = Point{ x: 0., y: 0. };
	let p2 = Point{ x: 1., y: 1. };
	let p3 = Point{ x: 1., y: 1. };
	let p4 = Point{ x: 1., y: 1. };
	let p5 = Point{ x: 0., y: 1. };
	let s = Shape::new(vec![p0,p1,p2,p3,p4,p5]);

	assert_eq!(s.edges.len(), 3);
}

#[test]
fn shape_overlap() {
	use std::f64::consts::PI;

	let p1 = Point{ x: 0., y: 0. };
	let p2 = Point{ x: 1., y: 1. };
	let p3 = Point{ x: 0., y: 1. };
	let s1 = Shape::new(vec![p1,p2,p3]);

	let (o,_,_) = Shape::overlap(0.,0.,0.,&s1,2.,0.,0.,&s1);
	assert!(!o);

	let (o,_,_) = Shape::overlap(0.,0.,0.,&s1,0.5,0.,0.,&s1);
	assert!(o);

	let (o,_,_) = Shape::overlap(0.,0.,0.,&s1,1.1,1.,PI,&s1);
	assert!(!o);

	let (o,_,_) = Shape::overlap(0.,0.,0.,&s1,0.9,1.,PI,&s1);
	assert!(o);
}
