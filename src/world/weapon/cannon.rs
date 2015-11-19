extern crate rand;

use world::World;
use world::event::{ EventSettings, EventArgs };
use world::body::BodyType;
use self::rand::Rng;

const LENGTH: f64 = 200.;
const SPREAD_ANGLE: f64 = 0.3;
const SPLIT: u32 = 4;
const MASK: u32 = 0;
const GROUP: u32 = 0;
const DELTA_LENGTH: f64 = 0.1;
const ORIGIN_DISTANCE: f64 = 0.5; //every ray are in one point at this distance (origin of shoots)
const DAMAGE: f64 = 1.;
const RELOAD_TIME: f64 = 0.5;

pub static RELOAD: &'static (Fn(&mut World, EventArgs)) = &reload;

pub struct Cannon {
	pub magazin: u32,
	pub state: CannonState,
}

fn reload (world: &mut World, args: EventArgs) {
	if let EventArgs::Usize1(id) = args {
		if let Some(body) = world.bodies.get_mut(&id) {
			if let BodyType::Character(ref mut character) = body.body_type {
				if character.cannon.magazin > 0 {
					character.cannon.state = CannonState::CHARGED;
				}
			}
		}
	}
}

pub enum CannonState {
	RELOADING,
	CHARGED,
}

impl Cannon {
	pub fn new() -> Cannon {
		Cannon {
			magazin: 10,
			state: CannonState::CHARGED,
		}
	}

	pub fn shoot(world: &mut World, id: usize) {

		let mut shoot = false;;
		let mut x = 0.;
		let mut y = 0.;
		let mut angle = 0.;
		if let Some(body) = world.bodies.get_mut(&id) {
			x = body.x();
			y = body.y();

			if let BodyType::Character(ref mut character) = body.body_type {
				angle = character.aim;

				if let CannonState::CHARGED = character.cannon.state {
					shoot = true;
					character.cannon.magazin -= 1;
					character.cannon.state = CannonState::RELOADING;
				}
			}
		}

		if shoot {
			let mut rng = rand::thread_rng();

			for _ in 1..SPLIT {
				let delta_angle = rng.gen_range(-SPREAD_ANGLE/2., SPREAD_ANGLE/2.);
				world.add_line_to_render_debug(x,y,x+LENGTH*(angle+delta_angle).cos(),y+LENGTH*(angle+delta_angle).sin(),1.);
				world.raycast(x,y,LENGTH,angle+delta_angle,MASK,GROUP,DELTA_LENGTH,|_length,body| -> bool {
					body.add_life(-DAMAGE);
					true
				});
			}

			world.add_event( EventSettings {
				delta_time: RELOAD_TIME,
				execute: RELOAD,
				args: EventArgs::Usize1(id)
			});
		}
	}
}
