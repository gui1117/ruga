use world::World;
use world::event::{ EventSettings, EventArgs };
use world::body::{ BodySettings, BodyType, CollisionType };
use world::geometry::{ Point, Shape };
use std::f64::consts::PI;

const LENGTH: f64 = 200.;
const STOP_TIME: f64 = 1.;
const EXPLODE_TIME: f64 = 1.; //since it stopped
const SPLIT: u32 = 10;
const MASK: u32 = 0;
const GROUP: u32 = 0;
const DELTA_LENGTH: f64 = 0.1;
const DAMAGE: f64 = 1.;
const RELOAD_TIME: f64 = 0.5;
const VELOCITY: f64 = 500.;

pub static RELOAD: &'static (Fn(&mut World, EventArgs)) = &reload;
pub static STOP: &'static (Fn(&mut World, EventArgs)) = &stop;
pub static EXPLODE: &'static (Fn(&mut World, EventArgs)) = &explode;

pub struct GrenadeLauncher {
	pub magazin: u32,
	pub state: GrenadeLauncherState,
}

fn reload (world: &mut World, args: EventArgs) {
	if let EventArgs::Usize1(id) = args {
		if let Some(body) = world.bodies.get_mut(&id) {
			if let BodyType::Character(ref mut character) = body.body_type {
				if character.grenade_launcher.magazin > 0 {
					character.grenade_launcher.state = GrenadeLauncherState::CHARGED;
				}
			}
		}
	}
}

fn stop (world: &mut World, args: EventArgs) {
	if let EventArgs::Usize1(id) = args {
		let mut explode = false;
		if let Some(body) = world.bodies.get_mut(&id) {
			explode = true;
			body.set_velocity(0.)
		}

		if explode {
			world.add_event( EventSettings {
				delta_time: EXPLODE_TIME,
				execute: EXPLODE,
				args: EventArgs::Usize1(id),
			});
		}
	}
}

fn explode (world: &mut World, args: EventArgs) {
	if let EventArgs::Usize1(id) = args {
		let mut x = 0.;
		let mut y = 0.;
		let mut explode = false;
		if let Some(body) = world.bodies.get_mut(&id) {
			x = body.x();
			y = body.y();
			explode = true;
		}

		if explode {
			for i in 0..SPLIT {
				let angle = (i as f64)*2.*PI/(SPLIT as f64);

				world.add_line_to_render_debug(x,y,x+LENGTH*(angle).cos(),y+LENGTH*(angle).sin(),1.);
				world.raycast(x,y,LENGTH,angle,MASK,GROUP,DELTA_LENGTH,|_length,body| -> bool {
					body.add_life(-DAMAGE);
					true
				});
			}
		}
	}
}

pub enum GrenadeLauncherState {
	RELOADING,
	CHARGED,
}

impl GrenadeLauncher {
	pub fn new() -> GrenadeLauncher {
		GrenadeLauncher {
			magazin: 10,
			state: GrenadeLauncherState::CHARGED,
		}
	}

	pub fn shoot(world: &mut World, id: usize) {

		let mut shoot = false;;
		let mut body_x = 0.;
		let mut body_y = 0.;
		let mut body_angle = 0.;
		let mut body_distance = 0.;
		if let Some(body) = world.bodies.get_mut(&id) {
			body_x = body.x();
			body_y = body.y();

			if let BodyType::Character(ref mut character) = body.body_type {
				body_angle = character.aim;
				body_distance = character.distance;

				if let GrenadeLauncherState::CHARGED = character.grenade_launcher.state {
					shoot = true;
					character.grenade_launcher.magazin -= 1;
					character.grenade_launcher.state = GrenadeLauncherState::RELOADING;
				}
			}
		}

		if shoot {
			let x = body_x+body_distance*(body_angle).cos();
			let y = body_y+body_distance*(body_angle).sin();

			world.add_line_to_render_debug(x,y,x+10.*(body_angle).cos(),y+10.*(body_angle).sin(),1.);

			let grenade_id = world.add_body( BodySettings {
				mask: 0,
				weight: 1.,
				life: 1.,
				group: 1,
				x: x,
				y: y,
				velocity: VELOCITY,
				angle: body_angle,
				shape: Shape::new(vec![
								  Point {x:-10.,y:-10.},
								  Point {x:10.,y:-10.},
								  Point {x:10.,y:10.},
								  Point {x:-10.,y:10.}
				]),
				body_type: BodyType::Grenade,
				collision_type: CollisionType::Bounce,
			});

			world.add_event( EventSettings {
				delta_time: STOP_TIME,
				execute: STOP,
				args: EventArgs::Usize1(grenade_id)
			});

			world.add_event( EventSettings {
				delta_time: RELOAD_TIME,
				execute: RELOAD,
				args: EventArgs::Usize1(id)
			});
		}
	}
}
