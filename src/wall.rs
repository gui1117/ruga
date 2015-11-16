use geometry::{ Shape, Point };
use body::{ BodySettings, BodyType };
use std::f64::INFINITY;

pub fn new() -> BodySettings {
	BodySettings {
		mask: 0,
		weight: INFINITY,
		group: 2,
		x: 0.,
		y: 0.,
		velocity: 0.,
		angle: 0.,
		shape: Shape::new(vec![
						  Point {x:20.,y:20.},
						  Point {x:80.,y:20.},
						  Point {x:80.,y:80.},
						  Point {x:20.,y:80.}
		]),
		body_type: BodyType::Wall,
	}
}
