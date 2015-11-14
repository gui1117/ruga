use std::cmp::Ordering;
use std::collections::{ HashMap, BinaryHeap };
use body::{ Body, BodySettings, BodyCollision };
use quadtree::{ Quadtree, Identifiable };
use geometry::Point;

pub struct Event {
	date: f64,
	msg: EventMessage,
}

pub enum EventMessage {
	Toto,
}

pub struct World {
	time: f64,
	next_id: usize,
	bodies: HashMap<usize,Body>,
	events: BinaryHeap<Event>,
	downleft: Point,
	width: f64,
	height: f64,
	quadtree_max_object: usize,
	quadtree_deepness: usize,
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
			quadtree_max_object: 5,
			quadtree_deepness: 10,
		}
	}

	pub fn add_body(&mut self, settings: BodySettings) -> usize {
		self.bodies.insert(self.next_id, Body::new(self.next_id,settings));

		self.next_id += 1;

		self.next_id - 1
	}

	pub fn update(&mut self , dt: f64) {
		for (_,body) in self.bodies.iter_mut() {
			body.update(dt);
		}

		let mut collision_possible: Vec<(usize,Vec<usize>)> = vec![];
		{
			let mut quadtree: Quadtree<Body> = Quadtree::new(self.downleft.x,self.downleft.y,self.width,self.height, self.quadtree_max_object, self.quadtree_deepness);

			for body in self.bodies.values() {
				collision_possible.push((body.id(),quadtree.insert(&body)));
			}
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
	}

	pub fn query<F: Fn(&mut Body)> (body: &Body, callback: F) {
		//TODO
	}

	pub fn raycast<F: Fn(Body) -> bool> (x: f64, y: f64, length: f64, angle: f64, mask: u32, group: u32, delta_lenght: f64, callback: F) {
		//TODO
	}

	//TODO pub fn segmentcast(segment: &Segment...

	pub fn add_event(&mut self, delta_time: f64, event_msg: EventMessage) {
		self.events.push(Event {
			date: self.time+delta_time,
			msg: event_msg,
		});
	}

	//	pub fn render(gl?) {
	//	TODO
	//	}
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
