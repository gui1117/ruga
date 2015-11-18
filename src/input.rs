extern crate piston;
use App;
use Direction;
use std::f64::consts::PI;

use body::BodyType;
use piston::input::{
	Button,
	Key,
	MouseButton,
	//	JoystickButton,
	//	Motion,
	//	JoystickAxisArgs,
};

impl App {
	fn update_character_direction(&mut self) {
		if let Some(id) = self.character_id {
			if let Some(character) = self.world.bodies.get_mut(&id) {
				if let Some(dir) = self.character_dir.last() {

					character.set_velocity(300.);

					let mut last_perpendicular: Option<&Direction> = None;
					for d in &self.character_dir {
						if d.perpendicular(dir) {
							last_perpendicular = Some(d);
						}
					}

					match dir {
						&Direction::Up => {
							match last_perpendicular {
								Some(&Direction::Left) => character.set_angle(-3.*PI/4.),
								Some(&Direction::Right) => character.set_angle(-PI/4.),
								_ => character.set_angle(-PI/2.),
							}
						},
						&Direction::Down => {
							match last_perpendicular {
								Some(&Direction::Left) => character.set_angle(3.*PI/4.),
								Some(&Direction::Right) => character.set_angle(PI/4.),
								_ => character.set_angle(PI/2.),
							}
						},
						&Direction::Right => {
							match last_perpendicular {
								Some(&Direction::Up) => character.set_angle(-PI/4.),
								Some(&Direction::Down) => character.set_angle(PI/4.),
								_ => character.set_angle(0.),
							}
						},
						&Direction::Left => {
							match last_perpendicular {
								Some(&Direction::Up) => character.set_angle(-3.*PI/4.),
								Some(&Direction::Down) => character.set_angle(3.*PI/4.),
								_ => character.set_angle(PI),
							}
						},
					}

				} else {

					character.set_velocity(0.);

				}
			}
		}
	}

	pub fn press(&mut self, button: &Button) {
		match *button {

			Button::Keyboard(key) => {
				let mut update_direction = false;
				match key {
					Key::Z => {
						if let Some(&Direction::Up) = self.character_dir.last() {
						} else {
							self.character_dir.push(Direction::Up);
							update_direction = true;
						}
					},
					Key::S => {
						if let Some(&Direction::Down) = self.character_dir.last() {
						} else {
							self.character_dir.push(Direction::Down);
							update_direction = true;
						}
					},
					Key::Q => {
						if let Some(&Direction::Left) = self.character_dir.last() {
						} else {
							self.character_dir.push(Direction::Left);
							update_direction = true;
						}
					},
					Key::D => {
						if let Some(&Direction::Right) = self.character_dir.last() {
						} else {
							self.character_dir.push(Direction::Right);
							update_direction = true;
						}
					},
					Key::Escape => { 
						self.quit = true; 
					},
					_ => (),
				}

				if update_direction {
					self.update_character_direction();
				}
			},

			Button::Joystick(joystick_button) => {
				match joystick_button.button {
					a @ _ => println!("j:{:?}",a),
				}
			},

			Button::Mouse(mouse_button) => {
				match mouse_button {
					MouseButton::Left => {

						if let Some(id) = self.character_id {
							let mut opt_event = None;
							if let Some(character_body) = self.world.bodies.get(&id) {
								if let BodyType::Character(ref character) = character_body.body_type {
									opt_event = character.cannon.shoot(0.,0.,0.);
								}
							}
							if let Some(event) = opt_event {
								self.world.add_event(0.,event);
							}
						}

					},
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
						self.character_dir.retain(|dir|{
							if let &Direction::Up = dir {
								return false;
							}
							true
						});
						update_direction = true;
					},
					Key::S => {
						self.character_dir.retain(|dir|{
							if let &Direction::Down = dir {
								return false;
							}
							true
						});
						update_direction = true;
					},
					Key::Q => {
						self.character_dir.retain(|dir|{
							if let &Direction::Left = dir {
								return false;
							}
							true
						});
						update_direction = true;
					},
					Key::D => {
						self.character_dir.retain(|dir|{
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
					self.update_character_direction();
				}
			},

			Button::Joystick(_joystick_button) => (),
			Button::Mouse(_mouse_button) => ()
		}

	}
}

