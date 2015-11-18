pub use self::quadtree::{ 
	Quadtree, 
	Identifiable, 
	FixedQuadtree,
	Localisable,
};

mod quadtree;

use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::cmp::Ordering;
use std::collections::{ HashMap, BinaryHeap };
use body::{ Body, BodySettings, BodyCollision };
use geometry::Point;
use camera::Camera;

pub struct Event {
	date: f64,
	execute: &'static Fn(&mut World),
}

//enum EventArgs {
//	fvec4([f64;4]),
//}

pub struct World {
	time: f64,
	next_id: usize,
	pub bodies: HashMap<usize,Body>,
	events: BinaryHeap<Event>,
	downleft: Point,
	width: f64,
	height: f64,
	quadtree_max_object: usize,
	quadtree_deepness: usize,
	camera: Camera,
	fixed_quadtree: FixedQuadtree,
}

impl World {
	pub fn new(x: f64, y: f64, height: f64, width: f64) -> World {
		World {
			time: 0.,
			next_id: 0,
			bodies: HashMap::new(),
			events: BinaryHeap::new(),
			downleft: Point {
				x: x,
				y: y,
			},
			width: width,
			height: height,
			quadtree_max_object: 1,
			quadtree_deepness: 10,
			camera: Camera {
				x: 0.,
				y: 0.,
				zoom: 1.,
				width: 640.,
				height: 480.,
			},
			fixed_quadtree: FixedQuadtree::new(0.,0.,500.,500.),
		}
	}

	pub fn add_body(&mut self, settings: BodySettings) -> usize {
		self.bodies.insert(self.next_id, Body::new(self.next_id,settings));

		self.next_id += 1;

		self.next_id - 1
	}

	pub fn update_camera(&mut self, args: &RenderArgs, character_id: Option<usize>) {
		self.camera.width = args.width as f64;
		self.camera.height = args.height as f64;

		if let Some(id) = character_id {
			if let Some(character_body) = self.bodies.get(&id) {
				self.camera.x = character_body.x();
				self.camera.y = character_body.y();
			}
		}
	}

	pub fn update(&mut self , dt: f64) {

		// execute event
		while let Some(&Event { date, execute:_ }) = self.events.peek() {
			if date > self.time {
				break;
			}
			if let Some(Event { date:_, execute }) = self.events.pop() {
				execute(self);
			}
		}

		// update bodies
		for (_,body) in self.bodies.iter_mut() {
			body.update(dt);
		}

		// resolve collision
		let mut collision_possible: Vec<(usize,Vec<usize>)> = vec![];
		{
			let mut quadtree: Quadtree<Body> = Quadtree::new(self.downleft.x,self.downleft.y,self.width,self.height, self.quadtree_max_object, self.quadtree_deepness);

			for body in self.bodies.values() {
				collision_possible.push((body.id(),quadtree.insert(&body)));
			}
			self.fixed_quadtree = quadtree.fix();
		}

		let mut collision: Vec<(usize,BodyCollision)> = vec![];
		for (a_id,a_col) in collision_possible {
			for b_id in a_col {
				if let Some(a) = self.bodies.get(&a_id) {
					if  let Some(b) = self.bodies.get(&b_id) {
						if ((a.mask() & b.group()) | (b.mask() & a.group())) != 0 {
							continue;
						}

						// we could test bounding box first
						// for better performance

						let overlap = Body::overlap(&a,&b);
						if overlap.overlap {
							let (solv_col_a,solv_col_b) = Body::collision(&a,&b,overlap);
							collision.push((a_id,solv_col_a));
							collision.push((b_id,solv_col_b));
						}
					}
				}
			}
		}
		for (id,col) in collision {
			if let Some(x) = self.bodies.get_mut(&id) {
				x.resolve_collision(col);
			}
		}

		// update time
		self.time += dt;
	}

	pub fn query<F: Fn(&mut Body, &mut Body)> (&mut self, a: &mut Body, callback: F) {
		let collision_possible = self.fixed_quadtree.query(a);
		for id in &collision_possible {

			if let Some(b) = self.bodies.get_mut(id) {

				if ((a.mask() & b.group()) | (b.mask() & a.group())) != 0 {
					continue;
				}

				// we could test bounding box first
				// for better performance

				let overlap = Body::overlap(a,&b);
				if overlap.overlap {
					callback(a,b);
				}

			}

		}
	}

	pub fn raycast<F: Fn(f64, &mut Body) -> bool> (&mut self, x: f64, y: f64, length: f64, angle: f64, mask: u32, group: u32, delta_length: f64, callback: F) {

		let mut a = Point { x: x, y: y};

		let delta_x = delta_length*angle.cos();
		let delta_y = delta_length*angle.sin();

		let nbr_iteration = (length/delta_length).floor() as usize;
		for i in 0..nbr_iteration {

			let collision_possible = self.fixed_quadtree.query(&a);

			for id in &collision_possible {

				if let Some(b) = self.bodies.get_mut(id) {

					if ((mask & b.group()) | (b.mask() & group)) != 0 {
						continue;
					}

					// we could test bounding box first
					// for better performance

					if a.in_shape(b.x(),b.y(),b.angle(),&b.shape()) {
						if callback(delta_length*(i as f64),b) == false {
							return;
						}
					}

				}

			}
			a.x += delta_x;
			a.y += delta_y;
		}
	}

	//TODO pub fn segmentcast(segment: &Segment...

	pub fn add_event(&mut self, delta_time: f64, execute: &'static Fn(&mut World)) {
		self.events.push(Event {
			date: self.time+delta_time,
			execute: execute,
		});
	}

	pub fn render_debug(&self, args: &RenderArgs, gl: &mut GlGraphics) {

		for body in self.bodies.values() {
			body.render_debug(args,&self.camera,gl);
		}

		if true {
			let mut quadtree: Quadtree<Body> = Quadtree::new(
				self.downleft.x,
				self.downleft.y,
				self.width,
				self.height, 
				self.quadtree_max_object, 
				self.quadtree_deepness);

			for body in self.bodies.values() {
				quadtree.insert(&body);
			}

			self.fixed_quadtree.render_debug(args,&self.camera,gl);
		}
	}
}

impl PartialEq for Event {
	fn eq(&self, other: &Self) -> bool {
		self.date == other.date
	}

	fn ne(&self, other: &Self) -> bool {
		self.date != other.date
	}
}

impl Eq for Event {
}

impl PartialOrd for Event {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		(-self.date).partial_cmp(&-other.date)
	}

	fn lt(&self, other: &Self) -> bool {
		self.date.gt(&other.date)
	}

	fn le(&self, other: &Self) -> bool {
		self.date.ge(&other.date)
	}

	fn gt(&self, other: &Self) -> bool {
		self.date.lt(&other.date)
	}

	fn ge(&self, other: &Self) -> bool {
		self.date.le(&other.date)
	}
}

impl Ord for Event {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.date < other.date {
			return Ordering::Greater;
		} else if self.date > other.date {
			return Ordering::Less;
		} else {
			return Ordering::Equal;
		}
	}
}
