use character;
use character::Character;
use quadtree::{ Identifiable, Localisable };
use geometry::{ Shape, Rectangle, Point };
use std::f64::consts::PI;

use graphics::context::Context;
use opengl_graphics::GlGraphics;

pub struct Body {
	id: usize,
	mask: u32,
	weight: f64,
	group: u32,
	x: f64,
	y: f64,
	velocity: f64,
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

impl BodyCollision {
	pub fn new() -> BodyCollision {
		BodyCollision {
			delta_velocity: 0.,
			delta_angle: 0.,
			delta_x: 0.,
			delta_y: 0.,
			body_type_collision: BodyTypeCollision::Nil,
		}
	}
}

pub enum BodyTypeCollision {
	Character(character::Collision),
	Nil,
}

pub struct BodySettings {
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

pub struct OverlapInformation {
	pub overlap: bool,
	pub length: f64,
	pub angle: f64,
}

impl Identifiable for Body {
	fn id(&self) -> usize {
		self.id
	}
}

impl Body {
	pub fn new(id: usize, b: BodySettings) -> Body {
		let mut body = Body {
			id: id,
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

	pub fn weight(&self) -> f64 {
		self.weight
	}

	pub fn set_weight(&mut self, weight: f64) {
		self.weight = weight;
	}

	pub fn add_weight(&mut self, weight: f64) {
		self.weight += weight;
	}

	pub fn mask(&self) -> u32 {
		self.mask
	}

	pub fn group(&self) -> u32 {
		self.group
	}

	pub fn x(&self) -> f64 {
		self.x
	}

	pub fn set_x(&mut self, x: f64) {
		self.x = x;
	}

	pub fn add_x(&mut self, x: f64) {
		self.x += x;
	}

	pub fn y(&self) -> f64 {
		self.y
	}

	pub fn set_y(&mut self, y: f64) {
		self.y = y;
	}

	pub fn add_y(&mut self, y: f64) {
		self.y += y;
	}

	pub fn velocity(&self) -> f64 {
		self.velocity
	}

	pub fn set_velocity(&mut self, velocity: f64) {
		self.velocity = velocity;
	}

	pub fn add_velocity(&mut self, velocity: f64) {
		self.velocity += velocity;
	}

	pub fn angle(&self) -> f64 {
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

	pub fn add_angle(&mut self, da: f64) {
		let a = self.angle();
		self.set_angle(a+da);
	}

	pub fn shape(&self) -> &Shape {
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

	pub fn overlap(a: &Body, b: &Body) -> OverlapInformation {
		let (overlap,length,angle) = Shape::overlap(a.x,a.y,a.angle,&a.shape,b.x,b.y,b.angle,&b.shape);
		OverlapInformation {
			overlap: overlap,
			length: length, 
			angle: angle,
		}
	}

	pub fn collision(a: &Body, b: &Body, info: OverlapInformation) -> (BodyCollision, BodyCollision) {
		(BodyCollision::new(),BodyCollision::new())
	}

	pub fn resolve_collision(&mut self, col: BodyCollision) {
		self.add_velocity(col.delta_velocity);
		self.add_angle(col.delta_angle);
		self.add_x(col.delta_x);
		self.add_y(col.delta_y);
		match self.body_type {
			BodyType::Character(ref mut character) => {
				if let BodyTypeCollision::Character(character_col) = col.body_type_collision {
					character.resolve_collision(character_col);
				}
			},
			BodyType::Nil => (),
		}
	}

	pub fn update(&mut self, dt: f64) {
		if self.velocity.abs() == 0. {
			return;
		}
		self.x += dt*self.velocity*self.angle.cos();
		self.y += dt*self.velocity*self.angle.sin();
	}

//	pub fn render_debug<F: FnOnce(Context,&mut GlGraphics)>(&self) -> F {
//		| contect, gl | {
//		}
//	}
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

#[test]
fn bounds_angle_shape() {
	let b = Body::new(12,BodySettings {
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
