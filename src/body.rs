use character;
use character::Character;
use collision_manager::{ Collidable, Identifiable, Localisable };
use std::f64::consts::PI;
use geometry::{ Shape, Rectangle, Point };

const DL: f64 = 1.;

pub struct Body {
	pub id: usize,
	pub mask: u32,
	pub weight: f64,
	pub group: u32,
	pub x: f64,
	pub y: f64,
	pub velocity: f64,
	angle: f64,
	bounds: Rectangle,
	shape: Shape,
	body_type: BodyType,
}

pub enum BodyType {
	Character(Character),
	Nil,
}

pub struct BodyCollision {
	delta_velocity: f64,
	delta_angle: f64,
	delta_x: f64,
	delta_y: f64,
	body_type_collision: BodyTypeCollision,
}

pub enum BodyTypeCollision {
	Character(character::Collision),
	Nil,
}

pub struct BodySettings {
	pub id: usize, 
	pub weight: f64,
	pub mask: u32, 
	pub group: u32, 
	pub x: f64, 
	pub y: f64, 
	pub velocity: f64, 
	pub angle: f64, 
	pub shape: Shape, 
	pub body_type: BodyType,
}

impl Body {
	pub fn new(b: BodySettings) -> Body {
		let mut body = Body {
			id: b.id,
			mask: b.mask,
			weight: b.weight,
			group: b.group,
			x: b.x,
			y: b.y,
			velocity: b.velocity,
			angle: b.angle,
			shape: Shape::new(vec![
							  Point {x:0.,y:1.},
							  Point {x:1.,y:1.},
							  Point {x:1.,y:0.}
			]),
			bounds: Rectangle {
				downleft:Point{x:0.,y:0.},
				width:0.,
				height:0.
			},
			body_type: b.body_type,
		};
		body.set_shape(b.shape);

		body
	}

	pub fn get_angle(&self) -> f64 {
		self.angle
	}

	pub fn set_angle(&mut self, a: f64) {
		self.angle = a;
		let (x_min,x_max) = self.shape.min_max(self.x,self.y,self.angle, 0.);
		let (y_min,y_max) = self.shape.min_max(self.x,self.y,self.angle, PI/2.);

		self.bounds = Rectangle {
			downleft: Point {
				x: x_min,
				y: y_min,
			},
			width: x_max-x_min,
			height: y_max-y_min,
		}
	}

	pub fn get_shape(&self) -> &Shape {
		&self.shape
	}

	pub fn set_shape(&mut self, s: Shape) {
		self.shape = s;

		let (x_min,x_max) = self.shape.min_max(self.x,self.y,self.angle, 0.);
		let (y_min,y_max) = self.shape.min_max(self.x,self.y,self.angle, PI/2.);

		self.bounds = Rectangle {
			downleft: Point {
				x: x_min,
				y: y_min,
			},
			width: x_max-x_min,
			height: y_max-y_min,
		}
	}

	pub fn basic_collision(a: &Body, a_col: &mut BodyCollision, b: &Body, b_col: &mut BodyCollision) {

		let a_dl = a.weight/(a.weight+b.weight)*DL;
		let b_dl = DL - a_dl;

		let ab_angle = (Point { x: b.x-a.x, y: b.y-a.y }).angle_0x();

		let a_dx = -a_dl*ab_angle.cos();
		let a_dy = -a_dl*ab_angle.sin();

		let b_dx = b_dl*ab_angle.cos();
		let b_dy = b_dl*ab_angle.sin();

		let mut i = 1.;

		while Shape::overlap(
			a.x+i*a_dx,
			a.y+i*a_dy,
			a.angle,
			&a.shape,
			b.x+i*b_dx,
			b.y+i*b_dy,
			b.angle,
			&b.shape) {

			i += 1.
		}

		a_col.delta_x = i*a_dx;
		a_col.delta_y = i*a_dy;
		b_col.delta_x = i*b_dx;
		b_col.delta_y = i*b_dy;
	}

//	pub fn get_elastic_collision(&self, other: &Body) -> (BodyCollision,BodyCollision) {
//	}
}


impl Identifiable for Body {
	fn get_id(&self) -> usize {
		self.id
	}
}

impl Localisable for Body {
	fn up(&self, y: f64) -> bool {
		self.bounds.downleft.y > y
	}
	fn down(&self, y: f64) -> bool {
		self.bounds.downleft.y + self.bounds.height < y
	}
	fn left(&self, x: f64) -> bool {
		self.bounds.downleft.x + self.bounds.width < x
	}
	fn right(&self, x: f64) -> bool {
		self.bounds.downleft.x > x
	}
}

impl Collidable for Body {
	type Collision = BodyCollision;

	fn get_mask(&self) -> u32 {
		self.mask
	}

	fn get_group(&self) -> u32 {
		self.group
	}

	fn collide(&self, other: &Self) -> bool {
		Shape::overlap(self.x,self.y,self.angle,&self.shape,other.x,other.y,other.angle,&other.shape)
	}

	fn update_position(&mut self, dt: f64) {
		if self.velocity.abs() == 0. {
			return;
		}
		self.x += dt*self.velocity*self.angle.cos();
		self.y += dt*self.velocity*self.angle.sin();
	}

	fn get_collision(&self, other: &Self) -> (Self::Collision,Self::Collision) {
		let a = &self;
		let b = other;

		let a_col = BodyCollision {
			delta_velocity: 0.,
			delta_angle: 0.,
			delta_x: 0.,
			delta_y: 0.,
			body_type_collision: BodyTypeCollision::Nil,
		};

		let b_col = BodyCollision {
			delta_velocity: 0.,
			delta_angle: 0.,
			delta_x: 0.,
			delta_y: 0.,
			body_type_collision: BodyTypeCollision::Nil,
		};

		match self.body_type {

			BodyType::Character(ref a_c) => {

				match other.body_type {
					BodyType::Character(ref b_c) => {

					},
					BodyType::Nil => {},
				}

			},
			BodyType::Nil => {},

		}

		(a_col,b_col)
	}

	fn solve_collision(&mut self, col: &Self::Collision) {
	}
}

#[test]
fn bounds_angle_shape() {
	let b = Body::new(BodySettings {
		id: 23,
		x: 2.,
		y: 2.,
		weight: 2.,
		mask: 1,
		group: 2,
		shape: Shape::new(vec![
						  Point {x:0.,y:5.},
						  Point {x:1.,y:5.},
						  Point {x:1.,y:0.}
		]),
		velocity: 3.,
		angle: PI/2.,
		body_type: BodyType::Character(Character {toto:43} ),
	});

	assert_eq!(b.bounds.downleft.x, -3.);
	assert_eq!(b.bounds.downleft.y, 2.);
	assert_eq!(b.bounds.width, 5.);
	assert_eq!(b.bounds.height, 1.);
}
