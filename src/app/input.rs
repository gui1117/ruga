use world::body::character::GunType;
use util::direction::Direction;
use super::point::Point;
use super::app::App;
use world::BodyTrait;
use glium::glutin::MouseButton;

use std::f64::consts::PI;

mod key {
    pub const Z:      u8 = 25;
    pub const Q:      u8 = 38;
    pub const S:      u8 = 39;
    pub const D:      u8 = 40;
    pub const E:      u8 = 26;
    pub const R:      u8 = 27;
    pub const T:      u8 = 28;
    pub const Y:      u8 = 29;
    pub const ESCAPE: u8 = 9;
}

impl App {
	fn update_player_direction(&mut self) {
		let mut velocity = 0.;
		let mut angle = 0.;
		if let Some(dir) = self.player_dir.last() {
			
			velocity = 1.;
			let mut last_perpendicular: Option<&Direction> = None;
			for d in &self.player_dir {
				if d.perpendicular(dir) {
					last_perpendicular = Some(d);
				}
			}

			match dir {
				&Direction::Up => {
					match last_perpendicular {
						Some(&Direction::Left) => angle = 3.*PI/4.,						
						Some(&Direction::Right) => angle = PI/4.,
						_ => angle = PI/2.,
					}
				},
				&Direction::Down => {
					match last_perpendicular {
						Some(&Direction::Left) => angle = -3.*PI/4.,
						Some(&Direction::Right) => angle = -PI/4.,
						_ => angle = -PI/2.,
					}
				},
				&Direction::Right => {
					match last_perpendicular {
						Some(&Direction::Down) => angle = -PI/4.,
						Some(&Direction::Up) => angle = PI/4.,
						_ => angle = 0.,
					}
				},
				&Direction::Left => {
					match last_perpendicular {
						Some(&Direction::Down) => angle = -3.*PI/4.,
						Some(&Direction::Up) => angle = 3.*PI/4.,
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

    pub fn key_pressed(&mut self, keycode: u8) {
        let mut update_direction = false;
        match keycode {
            key::Z => {
                if let Some(&Direction::Up) = self.player_dir.last() {
                } else {
                    self.player_dir.push(Direction::Up);
                    update_direction = true;
                }
            },
            key::S => {
                if let Some(&Direction::Down) = self.player_dir.last() {
                } else {
                    self.player_dir.push(Direction::Down);
                    update_direction = true;
                }
            },
            key::Q => {
                if let Some(&Direction::Left) = self.player_dir.last() {
                } else {
                    self.player_dir.push(Direction::Left);
                    update_direction = true;
                }
            },
            key::D => {
                if let Some(&Direction::Right) = self.player_dir.last() {
                } else {
                    self.player_dir.push(Direction::Right);
                    update_direction = true;
                }
            },
            key::ESCAPE => { 
                self.quit = true; 
            },
            key::E => {
                self.set_player_launch_grenade();
            }
            key::R => {
                self.set_player_next_gun(GunType::Rifle);
            }
            key::T => {
                self.set_player_next_gun(GunType::Shotgun);
            }
            key::Y => {
                self.set_player_next_gun(GunType::Sniper);
            }
            _ => (),
        }

        if update_direction {
            self.update_player_direction();
        }
    }

    pub fn key_released(&mut self, keycode: u8) {
        let mut update_direction = false;
        match keycode {
            key::Z => {
                self.player_dir.retain(|dir|{
                    if let &Direction::Up = dir {
                        return false;
                    }
                    true
                });
                update_direction = true;
            },
            key::S => {
                self.player_dir.retain(|dir|{
                    if let &Direction::Down = dir {
                        return false;
                    }
                    true
                });
                update_direction = true;
            },
            key::Q => {
                self.player_dir.retain(|dir|{
                    if let &Direction::Left = dir {
                        return false;
                    }
                    true
                });
                update_direction = true;
            },
            key::D => {
                self.player_dir.retain(|dir|{
                    if let &Direction::Right = dir{
                        return false;
                    }
                    true
                });
                update_direction = true;
            },
            key::ESCAPE => { 
                self.quit = true; 
            },
            _ => (),
        }

        if update_direction {
            self.update_player_direction();
        }
    }

    pub fn mouse_pressed(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.set_player_shoot(true),
            MouseButton::Right => self.set_player_attack_sword(),
            _ => (),
        }
    }

    pub fn mouse_released(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.set_player_shoot(false),
            _ => (),
        }
    }

    pub fn mouse_moved(&mut self, x: i32, y: i32) {
        let center = Point {
            x: self.window_size[0] as f64 / 2.,
            y: self.window_size[1] as f64 / 2.,
        };

        let cursor = Point {
            x: x as f64 - center.x,
            y: y as f64 - center.y,
        };

        self.set_player_aim(-cursor.angle_0x());
    }
}

