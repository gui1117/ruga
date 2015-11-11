extern crate collision_manager;

use collision_manager::*;

pub struct Body {
	id: usize,
	x: f64, // center
	y: f64, // center
	width: f64,
	height: f64,
	mask: u32,
	group: u32,
	dx: f64,
	dy: f64,
}

impl Localisable for Body {
	fn up(&self, y: f64) -> bool {
		self.y - self.height/2. > y
	}
	fn down(&self, y: f64) -> bool {
		self.y + self.height/2. < y
	}
	fn left(&self, x: f64) -> bool {
		self.x + self.width/2. < x
	}
	fn right(&self, x: f64) -> bool {
		self.x - self.width/2. > x
	}
}

impl Identifiable for Body {
	fn get_id(&self) -> usize {
		self.id
	}
}

impl Managable for Body {
	type Collision = (f64,f64); // vec to move

	fn get_mask(&self) -> u32 {
		self.mask
	}

	fn get_group(&self) -> u32 {
		self.group
	}

	fn collide(&self, other: &Self) -> bool {
		let a = &self;
		let b = other;

		(a.x-a.width/2. <= b.x+b.width/2.)
			&& (a.x+a.width/2. >= b.x-b.width/2.)
			&& (a.y-a.height/2. <= b.y+b.height/2.)
			&& (a.y+a.height/2. >= b.y-b.height/2.)
	}

	fn update_position(&mut self, dt: f64) {
		self.x += dt*self.dx;
		self.y += dt*self.dy;
	}

	fn get_collision(&self, other: &Self) -> (Self::Collision,Self::Collision) {
		let mut dx = ((self.width + other.width)/2. - (self.x - other.x).abs())/4.;
		let mut dy = ((self.width + other.width)/2. - (self.y - other.y).abs())/4.;
		if self.x < other.x {
			dx = -dx;
		}
		if self.y < other.y {
			dy = -dy;
		}
		((dx,dy),(-dx,-dy))
	}

	fn solve_collision(&mut self, col: &Self::Collision) {
		let &(dx,dy) = col;
		self.x += dx;
		self.y += dy;
	}
}

fn main() {
	//TODO
}
