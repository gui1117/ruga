use std::f64::consts::PI;
use world::World;
use world::event::{ EventSettings, EventArgs };

pub struct Cannon {
	pub magazin: u32,
	state: CannonState,
	length: f64,
	spread_angle: f64,
}

enum CannonState {
	RELOADING,
	CHARGED,
}

fn shoot (world: &mut World, args: EventArgs) {
	println!("bang!");
}

pub static SHOOT: &'static (Fn(&mut World, EventArgs)) = &shoot;

impl Cannon {
	pub fn new() -> Cannon {
		Cannon {
			magazin: 0,
			state: CannonState::CHARGED,
			length: 10.,
			spread_angle: PI/8.,
		}
	}

	pub fn shoot(&self, x: f64, y: f64, angle: f64) -> Option<EventSettings> {

		if let CannonState::CHARGED = self.state {
			return Some( EventSettings {
				delta_time: 0.,
				execute: SHOOT,
				args: EventArgs::Nil,
			});
		}

		None
	}
}
