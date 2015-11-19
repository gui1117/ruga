use world::geometry::{ Shape, Point };
use world::body::{ BodySettings, BodyType };
use world::weapon::cannon::Cannon;

pub struct Character {
	pub life: u32,
	pub aim: f64,
	pub cannon: Cannon,
	//pub grenade_lancher: GrenadeLauncher,
}

pub struct Collision {
	pub delta_life: u32,
	pub delta_aim: f64,
}

impl Character {
	pub fn new() -> BodySettings {
		BodySettings {
			mask: 0,
			life: 100.,
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

			body_type: BodyType::Character(Character {
				life: 10,
				aim: 0.,
				cannon: Cannon::new(),
			}),
		}
	}

	pub fn update(&mut self, _dt: f64) {
	}

	pub fn resolve_collision(&mut self, col: Collision) {
		self.life += col.delta_life;
	}
}
