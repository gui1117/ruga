use world::geometry::{ Shape, Point };
use world::body::{ BodySettings, BodyType };
use std::f64::INFINITY;

pub fn new(points: Vec<Point>) -> BodySettings {
	BodySettings {
		mask: 0,
		weight: INFINITY,
		group: 2,
		x: 0.,
		y: 0.,
		velocity: 0.,
		angle: 0.,
		shape: Shape::new(points),
		body_type: BodyType::Wall,
	}
}
