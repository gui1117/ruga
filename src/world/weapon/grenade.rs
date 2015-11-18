use geometry::{ Shape, Point };
use body::{ BodySettings, BodyType };

pub struct Grenade {
	radius: f64,
	split: u32,
	damage: u32,
}

impl Grenade {
	pub fn new() -> Body {
		//do call event in time to put velocity=0
		//do call event to do explosion
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
							  Point {x:-10.,y:10.}]),

			body_type: BodyType::Grenade(Grenade {{
				radius: radius,
				split: split,
				damage: damage,
			}),
		}
	}
}
