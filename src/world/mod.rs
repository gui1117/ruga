extern crate xml;

mod quadtree;

pub mod body;
pub mod weapon;
pub mod geometry;
pub mod camera;
pub mod collision_manager;
pub mod event;

use self::quadtree::{ FixedQuadtree, Quadtree, Identifiable };
use std::collections::{ HashMap, BinaryHeap };
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use self::camera::Camera;
use self::geometry::Point;
use self::body::{ Body, BodySettings, BodyCollision };
use self::event::{ Event, EventSettings };

pub struct World {
	time: f64,
	next_id: usize,
	pub bodies: HashMap<usize,Body>, //maybe an array is better if nothing is created
	events: BinaryHeap<Event>,
	downleft: Point,
	width: f64,
	height: f64,
	quadtree_max_object: usize,
	quadtree_deepness: usize,
	camera: Camera,
	fixed_quadtree: FixedQuadtree,
	debug_lines: Vec<DebugLine>,
}

struct DebugLine {
	x1:f64,
	y1:f64,
	x2:f64,
	y2:f64,
	time: f64,
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
			fixed_quadtree: FixedQuadtree::new(0.,0.,height,width),
			debug_lines: vec![],
		}
	}

	pub fn load(file_name: &str) -> Result<World,&str> {
		use std::fs::File;
		use std::io::BufReader;
		use self::xml::reader::{EventReader, XmlEvent};
		use std::f64;
		use std::str::FromStr;
		use self::body::wall as Wall;
		use self::body::character::Character;

		// get size of the world
		let file = File::open(file_name).unwrap();
		let file_reader= BufReader::new(file);
		let parser = EventReader::new(file_reader);
		let mut height = -1.;
		let mut width = -1.;
		let mut tile_height = -1.;
		let mut tile_width = -1.;
		for e in parser {
			match e {
				Ok(XmlEvent::StartElement { name, attributes, ..}) => {
					if name.local_name != "map" {
						continue;
					}
					for att in attributes {
						if att.name.local_name == "height" {
							height = f64::from_str(&*att.value).unwrap();
						} else if att.name.local_name == "width" {
							width = f64::from_str(&*att.value).unwrap();
						} else if att.name.local_name == "tilewidth" {
							tile_width = f64::from_str(&*att.value).unwrap();
						} else if att.name.local_name == "tileheight" {
							tile_height = f64::from_str(&*att.value).unwrap();
						}
					}
					break;
				}
				Err(e) => {
					return Err("parsing error");
				}
				_ => ()
			}
		}
		if width < 0. || height < 0. || tile_height < 0. || tile_width < 0. {
			return Err("tile or map width or height < 0.");
		}
		
		let mut world = World::new(0.,0.,width*tile_width, height*tile_height);

		// add object into the world
		let file = File::open(file_name).unwrap();
		let file_reader= BufReader::new(file);
		let parser = EventReader::new(file_reader);

		let mut layer = false;
		let mut layer_data = false;
		let mut tile_number = 0.;
		for e in parser {
			match e {
				Ok(XmlEvent::StartElement { name, attributes, ..}) => {
					if name.local_name == "layer" {
						layer = true;
					}
					if name.local_name == "data" && layer {
						tile_number = 0.;
						layer_data = true;
					}
					if name.local_name == "tile" && layer_data {
						for att in attributes {
							if att.name.local_name == "gid" {
								match &*att.value {
									"1" => {
										let x = tile_number%width;
										let y = tile_number/width;
										let p0 = Point {x: x*tile_width, y: y*tile_height};
										let p1 = Point {x: (x+1.)*tile_width, y: y*tile_height};
										let p2 = Point {x: (x+1.)*tile_width, y: (y+1.)*tile_height};
										let p3 = Point {x: x*tile_width, y: (y+1.)*tile_height};
										world.add_body(Wall::new(vec![p0,p1,p2,p3]));
									}
									"9" => {
//										let x = tile_number%width;
//										let y = tile_number/width;
//										world.add_body(Character::new(x,y));
									}
									_ => ()
								}
							}
						}
						tile_number += 1.;
					}
				}
				Ok(XmlEvent::EndElement { name }) => {
					if name.local_name == "layer" {
						layer = false;
					}
					if name.local_name == "data" {
						layer_data = false;
					}
				}
				Err(e) => {
					return Err("parsing error");
				}
				_ => ()
			}
		}

		Ok(world)
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
		// debug lines
		{
			let mut i = 0;
			while i < self.debug_lines.len() {
				self.debug_lines[i].time -= dt;
				if self.debug_lines[i].time <= 0. {
					self.debug_lines.remove(i);
				}
				i += 1;
			}
		}

		// execute event
		while let Some(&Event { date, execute:_, args:_ }) = self.events.peek() {
			if date > self.time {
				break;
			}
			if let Some(Event { date:_, execute, args }) = self.events.pop() {
				execute(self, args);
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

						if !Body::bounding_box_overlap(&a,&b) {
							continue;
						}

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

	/// not tested !
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

	/// the callback return true when stoping
	pub fn raycast<F: Fn(f64, &mut Body) -> bool> (&mut self, x: f64, y: f64, length: f64, angle: f64, immune: Vec<usize>, mask: u32, group: u32, delta_length: f64, callback: F) {

		let mut a = Point { x: x, y: y };

		let mut collisionned: Vec<usize> = vec![];

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

					for id in &immune {
						if *id == b.id() {
							continue;
						}
					}
					for id in &collisionned{
						if *id == b.id() {
							continue;
						}
					}

					// we could test bounding box first
					// for better performance

					if a.in_shape(b.x(),b.y(),b.angle(),&b.shape()) {
						if callback(delta_length*(i as f64),b) == false {
							return;
						}
						collisionned.push(b.id());
					}

				}

			}
			a.x += delta_x;
			a.y += delta_y;
		}
	}

	//TODO pub fn segmentcast(segment: &Segment...

	pub fn add_event(&mut self, settings: EventSettings) {
		self.events.push(Event {
			date: self.time+settings.delta_time,
			execute: settings.execute,
			args: settings.args,
		});
	}

	pub fn add_line_to_render_debug(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, time: f64) {
		self.debug_lines.push( DebugLine {
			x1:x1,
			y1:y1,
			x2:x2,
			y2:y2,
			time:time,
		});
	}

	pub fn render_debug(&self, args: &RenderArgs, gl: &mut GlGraphics) {

		{
			use graphics::Transformed;
			use graphics::line::{ 
				Line as LineDrawer, 
				Shape as LineShape,
			};
			use graphics::default_draw_state;

			const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0]; 

			let line_drawer = LineDrawer {
				color: BLUE,
				radius: 1.,
				shape: LineShape::Round,
			};

			gl.draw(args.viewport(), |context, gl| {
				let transform = self.camera.trans(context.transform);

				for line in &self.debug_lines {
					line_drawer.draw([line.x1,line.y1,line.x2,line.y2], default_draw_state(), transform, gl);
				}
			});

		}

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
