extern crate piston;
use App;
use Direction;
use std::f64::consts::PI;

use piston::input::{
	Button,
	Key,
	MouseButton,
	JoystickButton,
	Motion,
	JoystickAxisArgs,
};

impl App {
	pub fn press(&mut self, button: &Button) {
		match *button {

			Button::Keyboard(key) => {
				let mut update_direction = false;
				match key {
					Key::Z => {
						self.character_dir.push(Direction::Up);
						update_direction = true;
					},
					Key::S => {
						self.character_dir.push(Direction::Down);
						update_direction = true;
					},
					Key::Q => {
						self.character_dir.push(Direction::Left);
						update_direction = true;
					},
					Key::D => {
						self.character_dir.push(Direction::Right);
						update_direction = true;
					},
					Key::Escape => { 
						self.quit = true; 
					},
					_ => (),
				}

				if update_direction {
					if let Some(id) = self.character_id {
						if let Some(character) = self.world.bodies.get_mut(&id) {
							if let Some(dir) = self.character_dir.last() {

								character.set_velocity(100.);

								let len = self.character_dir.len();
								if len >= 2 {
									let dir_2 = &self.character_dir[len-2];
									match (*dir,*dir_2) {
										(Direction::Up,Direction::Left) => character.set_angle(-PI*3./4.),
										(Direction::Up,Direction::Right) => character.set_angle(-PI/4.),
										(Direction::Up,_) => character.set_angle(-PI/2.),

										(Direction::Down,Direction::Left) => character.set_angle(PI*3./4.),
										(Direction::Down,Direction::Right) => character.set_angle(PI/4.),
										(Direction::Down,_) => character.set_angle(PI/2.),

										(Direction::Left,Direction::Up) => character.set_angle(-PI*3./4.),
										(Direction::Left,Direction::Down) => character.set_angle(PI*3./4.),
										(Direction::Left,_) => character.set_angle(-PI),

										(Direction::Right,Direction::Up) => character.set_angle(-PI/4.),
										(Direction::Right,Direction::Down) => character.set_angle(PI/4.),
										(Direction::Right,_) => character.set_angle(0.),
									}
								} else {
									match *dir {
										Direction::Up => character.set_angle(-PI/2.),
										Direction::Down => character.set_angle(PI/2.),
										Direction::Left => character.set_angle(-PI),
										Direction::Right => character.set_angle(0.),
									}
								}

							} else {

								character.set_velocity(0.);

							}
						}
					}
				}
			},

			Button::Joystick(joystick_button) => {
				match joystick_button.button {
					a @ _ => println!("j:{:?}",a),
				}
			},

			Button::Mouse(mouse_button) => {
				println!("m:{:?}",mouse_button);
			},
		}
	}
}

