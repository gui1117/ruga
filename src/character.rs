use geometry::{ Shape, Point };
use body::{ BodySettings, BodyType };

pub struct Character {
	pub life: u32,
}

pub struct Collision {
	pub delta_life: u32,
}

impl Character {
	pub fn new() -> BodySettings {
		BodySettings {
			mask: 0,
			weight: 1.,
			group: 1,
			x: 0.,
			y: 0.,
			velocity: 0.,
			angle: 0.,
			shape: Shape::new(vec![
							  Point {x:-10.,y:-10.},
							  Point {x:10.,y:-10.},
							  Point {x:25.,y:0.},
							  Point {x:10.,y:10.},
							  Point {x:-10.,y:10.}
			]),
			body_type: BodyType::Character(Character {
				life: 10,
			}),
		}
	}

	pub fn resolve_collision(&mut self, col: Collision) {
		self.life += col.delta_life;
	}
}
