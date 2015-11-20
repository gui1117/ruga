use direction::Direction;
use world::geometry::Point;
use app::App;
use std::f64::consts::PI;
use piston::input::{
	Button,
	Key,
	MouseButton,
	Motion,
	//	JoystickButton,
	//	JoystickAxisArgs,
};

impl App {
	fn update_player_direction(&mut self) {
		let mut velocity = 0.;
		let mut angle = 0.;
		if let Some(dir) = self.player_dir.last() {
			
			velocity = 300.;
			let mut last_perpendicular: Option<&Direction> = None;
			for d in &self.player_dir {
				if d.perpendicular(dir) {
					last_perpendicular = Some(d);
				}
			}

			match dir {
				&Direction::Up => {
					match last_perpendicular {
						Some(&Direction::Left) => angle = -3.*PI/4.,
						Some(&Direction::Right) => angle = -PI/4.,
						_ => angle = -PI/2.,
					}
				},
				&Direction::Down => {
					match last_perpendicular {
						Some(&Direction::Left) => angle = 3.*PI/4.,						
						Some(&Direction::Right) => angle = PI/4.,
						_ => angle = PI/2.,
					}
				},
				&Direction::Right => {
					match last_perpendicular {
						Some(&Direction::Up) => angle = -PI/4.,
						Some(&Direction::Down) => angle = PI/4.,
						_ => angle = 0.,
					}
				},
				&Direction::Left => {
					match last_perpendicular {
						Some(&Direction::Up) => angle = -3.*PI/4.,
						Some(&Direction::Down) => angle = 3.*PI/4.,
						_ => angle = PI,
					}
				},
			}
		}
		self.set_player_velocity(velocity);
		if velocity != 0. {
			self.set_player_angle(angle);
		}

	}

	pub fn press(&mut self, button: &Button) {
		match *button {

			Button::Keyboard(key) => {
				let mut update_direction = false;
				match key {
					Key::Z => {
						if let Some(&Direction::Up) = self.player_dir.last() {
						} else {
							self.player_dir.push(Direction::Up);
							update_direction = true;
						}
					},
					Key::S => {
						if let Some(&Direction::Down) = self.player_dir.last() {
						} else {
							self.player_dir.push(Direction::Down);
							update_direction = true;
						}
					},
					Key::Q => {
						if let Some(&Direction::Left) = self.player_dir.last() {
						} else {
							self.player_dir.push(Direction::Left);
							update_direction = true;
						}
					},
					Key::D => {
						if let Some(&Direction::Right) = self.player_dir.last() {
						} else {
							self.player_dir.push(Direction::Right);
							update_direction = true;
						}
					},
					Key::Escape => { 
						self.quit = true; 
					},
					_ => (),
				}

				if update_direction {
					self.update_player_direction();
				}
			},

			Button::Joystick(joystick_button) => {
				match joystick_button.button {
					a @ _ => println!("j:{:?}",a),
				}
			},

			Button::Mouse(mouse_button) => {
				match mouse_button {
					MouseButton::Left => self.set_player_cannon_shoot(),
					MouseButton::Right => self.set_player_launch_grenade(),
					_ => (),
				}
			},
		}
	}

	pub fn release(&mut self, button: &Button) {
		match *button {

			Button::Keyboard(key) => {
				let mut update_direction = false;
				match key {
					Key::Z => {
						self.player_dir.retain(|dir|{
							if let &Direction::Up = dir {
								return false;
							}
							true
						});
						update_direction = true;
					},
					Key::S => {
						self.player_dir.retain(|dir|{
							if let &Direction::Down = dir {
								return false;
							}
							true
						});
						update_direction = true;
					},
					Key::Q => {
						self.player_dir.retain(|dir|{
							if let &Direction::Left = dir {
								return false;
							}
							true
						});
						update_direction = true;
					},
					Key::D => {
						self.player_dir.retain(|dir|{
							if let &Direction::Right = dir{
								return false;
							}
							true
						});
						update_direction = true;
					},
					Key::Escape => { 
						self.quit = true; 
					},
					_ => (),
				}

				if update_direction {
					self.update_player_direction();
				}
			},

			Button::Joystick(_joystick_button) => (),
			Button::Mouse(_mouse_button) => ()
		}

	}

	pub fn motion(&mut self, motion: &Motion) {
		match *motion {
			Motion::MouseCursor(x,y) => {
				let cursor = Point {
					x: x-self.window_size[0]/2.,
					y: y-self.window_size[1]/2.,
				};
				self.set_player_aim(cursor.angle_0x());
			},
			_ => (),
		}
	}
}

