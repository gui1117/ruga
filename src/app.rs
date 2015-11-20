extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

use world::body::BodyType;
use world::weapon::cannon::Cannon;
use world::weapon::grenade_launcher::GrenadeLauncher;
use direction::Direction;
use world::World;
use std::f64;
use piston::input::{ 
	RenderArgs, 
	UpdateArgs, 
	UpdateEvent,
};

pub struct App {
	pub gl: opengl_graphics::GlGraphics,
	pub world: World,
	pub quit: bool,
	pub player_id: Option<usize>,
	pub player_dir: Vec<Direction>,
	pub window_size: [f64;2],
}

impl App {
	pub fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

		self.world.update_camera(args,self.player_id);

		const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

		self.gl.draw(args.viewport(), |_, gl| {
			graphics::clear(BLACK, gl);
		});

		self.world.render_debug(args,&mut self.gl);
	}

	pub fn update(&mut self, args: &UpdateArgs) {
		self.world.update(args.dt);
	}

	//interface
	pub fn player_aim(&self) -> f64 {
		if let Some(id) = self.player_id {
			if let Some(character_body) = self.world.bodies.get(&id) {
				if let BodyType::Character(ref character) = character_body.body_type {
					return character.aim;
				}
			}
		}

		f64::NAN
	}

	pub fn set_player_aim(&mut self, aim: f64) {
		if let Some(id) = self.player_id {
			if let Some(character_body) = self.world.bodies.get_mut(&id) {
				if let BodyType::Character(ref mut character) = character_body.body_type {
					character.aim = aim;
				}
			}
		}
	}

	pub fn player_velocity(&self) -> f64 {
		if let Some(id) = self.player_id {
			if let Some(body) = self.world.bodies.get(&id) {
				body.velocity();
			}
		}

		f64::NAN
	}

	pub fn set_player_velocity(&mut self, v: f64) {
		if let Some(id) = self.player_id {
			if let Some(body) = self.world.bodies.get_mut(&id) {
				body.set_velocity(v);
			}
		}
	}

	pub fn player_angle(&self) -> f64 {
		if let Some(id) = self.player_id {
			if let Some(body) = self.world.bodies.get(&id) {
				body.angle();
			}
		}

		f64::NAN
	}

	pub fn set_player_angle(&mut self, a: f64) {
		if let Some(id) = self.player_id {
			if let Some(body) = self.world.bodies.get_mut(&id) {
				body.set_angle(a);
			}
		}
	}

	pub fn set_player_cannon_shoot(&mut self) {
		if let Some(id) = self.player_id {
			Cannon::shoot(&mut self.world, id);
		}
	}

	pub fn set_player_launch_grenade(&mut self) {
		if let Some(id) = self.player_id {
			GrenadeLauncher::shoot(&mut self.world, id);
		}
	}
}

