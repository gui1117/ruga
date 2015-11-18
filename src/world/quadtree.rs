extern crate graphics;

use camera::Camera;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::fmt;

/// trait required by objects inserted in the quadtree,
/// used when returning the possible collision of an object
pub trait Identifiable {
	fn id(&self) -> usize;
}

/// trait required by objects inserted in the quadtree,
/// used when inserting an object in the quadtree,
/// an object is consider up to a certain y when its 
/// bounding box is **entirely** up to this y.
/// Same for down, left and right...
pub trait Localisable {
	fn up (&self, f64) -> bool;
	fn down (&self, f64) -> bool;
	fn left (&self, f64) -> bool;
	fn right (&self, f64) -> bool;
}

// enum used when classifying an object in the sons of 
// a node.
enum Quadrant {
	Upleft,
	Upright,
	Downleft,
	Downright,
	Nil,
}

// quadrant! take 3 parametre:
// * the x coordinate for the vertical axe
// * the y coordinate for the horizontal axe 
// * a Localisable object
macro_rules! quadrant {
	($x: ident, $y: ident, $obj: ident) => (
		{
			if $obj.up($y) {
				if $obj.right($x) {
					Quadrant::Upright
				} else if $obj.left($x) {
					Quadrant::Upleft
				} else {
					Quadrant::Nil
				}
			} else if $obj.down($y) {
				if $obj.right($x) {
					Quadrant::Downright
				} else if $obj.left($x) {
					Quadrant::Downleft
				} else {
					Quadrant::Nil
				}
			} else {
				Quadrant::Nil
			}
		}
	)
}

/// Quadtree represent a node of the quadtree,
///
/// * level is the level where it cannot split anymore,
/// * max_object is the number of object necessar to split the node,
/// * level is the level of the node,
/// * objects contains all the objects of the node if the node isn't splited
/// otherwise it contains all the objects that cannot be stored in sons' node.
/// * is the bounding box of the node
/// * nodes are its sons if they are.
/// * x and y are downleft coordinate
///
/// you must be careful with lifetime: for performance purpose the quadtree 
/// borrow objects and store this borrow. that's why objects must live longer
/// than quadtree. And those objects will be mutable again when the quadtree is
/// gone
pub struct Quadtree<'l, T: 'l + Localisable + Identifiable> {
	max_object: usize,
	level: usize,
	objects: Vec<&'l T>,
	x: f64, //downleft
	y: f64, //downleft
	width: f64,
	height: f64,
	nodes: QuadtreeBatch<'l, T>,
}

// QuadtreeBatch represent the sons of a node,
// sons can be none -> Nil,
// otherwise they're 4 -> Cons(..,..,..,..)
enum QuadtreeBatch<'l, T: 'l + Localisable + Identifiable> {
	Cons {
		upleft: Box<Quadtree<'l, T>>,
		upright: Box<Quadtree<'l, T>>,
		downright: Box<Quadtree<'l, T>>,
		downleft: Box<Quadtree<'l, T>>,
	},
	Nil,
}



impl<'l, T: 'l + Localisable + Identifiable> Quadtree<'l, T> {
	/// create a new Quadtree 
	pub fn new(x: f64, y: f64, width: f64, height: f64, max_obj: usize, max_lvl: usize) -> Quadtree<'l, T> {

		Quadtree {
			level: max_lvl,
			max_object: max_obj,

			objects: vec![],

			x: x,
			y: y,

			width: width,
			height: height,

			nodes: QuadtreeBatch::Nil,
		}
	}

	// split the Quadtree: creates its 4 sons 
	fn split(&mut self) {
		let max_lvl = self.level;
		let max_obj = self.max_object;

		let sub_width = self.width/2.;
		let sub_height = self.height/2.;
		let x = self.x;
		let y = self.y;

		self.nodes = QuadtreeBatch::Cons {
			downleft:Box::new(Quadtree::new(x, y, sub_width, sub_height, max_obj, max_lvl-1)),
			downright:Box::new(Quadtree::new(x+sub_width, y, sub_width, sub_height, max_obj, max_lvl-1)),
			upright:Box::new(Quadtree::new(x+sub_width, y+sub_height, sub_width, sub_height, max_obj, max_lvl-1)),
			upleft:Box::new(Quadtree::new(x, y+sub_height, sub_width, sub_height, max_obj, max_lvl-1)),
		};
	}

	// return a Vec of Id of all the objects in the current Quadtree but
	// not in its sons 
	fn objects_ids(&self) -> Vec<usize> {
		let mut result = vec![];
		for obj in &self.objects {
			result.push(obj.id());
		}
		result
	}

	// return a Vec of Id of all the objects in the current Quadtree and 
	// in its sons
	fn all_objects_ids(&self) -> Vec<usize> {
		let mut result = vec![];
		for obj in &self.objects {
			result.push(obj.id());
		}
		if let QuadtreeBatch::Cons{ref upleft,ref upright,ref downleft,ref downright} = self.nodes {
			let r = upleft.all_objects_ids();
			for id in r {
				result.push(id);
			}

			let r = upright.all_objects_ids();
			for id in r {
				result.push(id);
			}

			let r = downright.all_objects_ids();
			for id in r {
				result.push(id);
			}

			let r = downleft.all_objects_ids();
			for id in r {
				result.push(id);
			}
		}
		result
	}

	/// insert an object in the Quadtree and return a Vec of Id of all
	/// the objects that can collide with it
	pub fn insert(&mut self, obj: &'l T) -> Vec<usize> {
		if self.level == 0 {
			let ids = self.objects_ids();
			self.objects.push(obj);
			return ids;
		}
		if let QuadtreeBatch::Nil = self.nodes {
			if self.max_object == self.objects.len() {

				self.split();

				let x = self.x + self.width/2.;
				let y = self.y + self.height/2.;
				let mut objects_remaining: Vec<&'l T> = vec![];
				if let QuadtreeBatch::Cons { ref mut upleft, ref mut upright, ref mut downright, ref mut downleft } = self.nodes {

					for obj in &self.objects {

						match quadrant!(x,y,obj) {
							Quadrant::Upright => { upright.insert(obj); },
							Quadrant::Upleft => { upleft.insert(obj); },
							Quadrant::Downright => { downright.insert(obj); },
							Quadrant::Downleft => { downleft.insert(obj); },
							Quadrant::Nil => { objects_remaining.push(obj); },
						}

					}

				}
				self.objects = objects_remaining;

			} else {
				let ids = self.objects_ids();
				self.objects.push(obj);
				return ids;
			}
		}

		let mut ids = self.objects_ids();
		let mut other_ids: Vec<usize> = vec![];

		if let QuadtreeBatch::Cons { ref mut upleft, ref mut upright, ref mut downright, ref mut downleft } = self.nodes {

			let x = self.x + self.width/2.;
			let y = self.y + self.height/2.;

			match quadrant!(x,y,obj) {
				Quadrant::Upright => { other_ids = upright.insert(obj); },
				Quadrant::Upleft => { other_ids = upleft.insert(obj); },
				Quadrant::Downright => { other_ids = downright.insert(obj); },
				Quadrant::Downleft => { other_ids = downleft.insert(obj); },
				Quadrant::Nil => { 
					let r = upright.all_objects_ids();
					for i in r {
						other_ids.push(i);
					}

					let r = upleft.all_objects_ids();
					for i in r {
						other_ids.push(i);
					}

					let r = downright.all_objects_ids();
					for i in r {
						other_ids.push(i);
					}

					let r = downleft.all_objects_ids();
					for i in r {
						other_ids.push(i);
					}
					self.objects.push(obj); 
				},
			}

		}

		for id in other_ids {
			ids.push(id);
		}
		ids
	}

	pub fn render_debug(&self, args: &RenderArgs, camera: &Camera, gl: &mut GlGraphics) {
		use graphics::Transformed;
		use graphics::line::{ 
			Line as LineDrawer, 
			Shape as LineShape,
		};
		use graphics::types::Line;
		use graphics::default_draw_state;

		if let QuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {
			const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.5]; 

			let line_drawer = LineDrawer {
				color: GREEN,
				radius: 1.,
				shape: LineShape::Round,
			};

			let mut lines: Vec<Line> = vec![];

			lines.push([
					   self.x,
					   self.y+self.height/2.,
					   self.x+self.width,
					   self.y+self.height/2.]);

			lines.push([
					   self.x+self.width/2.,
					   self.y,
					   self.x+self.width/2.,
					   self.y+self.height]);

			gl.draw(args.viewport(), |context, gl| {
				let transform = camera.trans(context.transform);

				for line in lines {
					line_drawer.draw(line, default_draw_state(), transform, gl);
				}
			});

			upleft.render_debug(args,camera,gl);
			downleft.render_debug(args,camera,gl);
			upright.render_debug(args,camera,gl);
			downright.render_debug(args,camera,gl);
		}
	}

	/// return a fixed quadtree that have the same structure as self but only store ids 
	/// and not point pointer on objects
	pub fn fix(&self) -> FixedQuadtree {
		let mut fixed = FixedQuadtree::new(self.x,self.y,self.width,self.height);
		fixed.ids = self.objects_ids();
	
		if let QuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {
			fixed.nodes = FixedQuadtreeBatch::Cons {
				downleft:Box::new(downleft.fix()),
				downright:Box::new(downright.fix()),
				upright:Box::new(upright.fix()),
				upleft:Box::new(upleft.fix()),
	
			};
		} 
	
		fixed
	}
}

impl<'l, T: 'l + Localisable + Identifiable> fmt::Debug for Quadtree<'l, T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for _ in 0..self.level {
			write!(f," ").unwrap();
		}
		write!(f,">x:{},y:{},width:{},height:{}\n",self.x,self.y,self.width,self.height).unwrap();
		for o in &self.objects {
			write!(f,"{:?}",o.id()).unwrap();
			write!(f,",").unwrap();
		}

		write!(f,"\n").unwrap();

		if let QuadtreeBatch::Cons{ref upleft,ref upright,ref downleft,ref downright} = self.nodes {
			write!(f,"dl: {:?}",downleft).unwrap();
			write!(f,"dr: {:?}",downright).unwrap();
			write!(f,"ul: {:?}",upleft).unwrap();
			write!(f,"ur: {:?}",upright).unwrap();
		}

		write!(f,"\n")
	}
}

/// like a quadtree but only store id instead of reference to objects
pub struct FixedQuadtree {
	ids: Vec<usize>,
	x: f64, //downleft
	y: f64, //downleft
	width: f64,
	height: f64,
	nodes: FixedQuadtreeBatch,
}

// like a nodebatch but for FixedQuadtree
enum FixedQuadtreeBatch {
	Cons {
		upleft: Box<FixedQuadtree>,
		upright: Box<FixedQuadtree>,
		downright: Box<FixedQuadtree>,
		downleft: Box<FixedQuadtree>,
	},
	Nil,
}

impl FixedQuadtree {
	/// create a new Quadtree 
	///
	/// * x,y: downleft
	/// * width,height: size 
	pub fn new(x: f64, y: f64, width: f64, height: f64) -> FixedQuadtree {
		FixedQuadtree {
			ids: vec![],
			x: x,
			y: y,
			width: width,
			height: height,
			nodes: FixedQuadtreeBatch::Nil,
		}
	}

	// return a Vec of Id of all the objects in the current Quadtree but
	// not in its sons 
	fn objects_ids(&self) -> Vec<usize> {
		let mut result = vec![];
		for id in &self.ids {
			result.push(*id);
		}
		result
	}

	// return a Vec of Id of all the objects in the current Quadtree and 
	// in its sons
	fn all_objects_ids(&self) -> Vec<usize> {
		let mut result = vec![];
		for id in &self.ids {
			result.push(*id);
		}

		if let FixedQuadtreeBatch::Cons{ref upleft,ref upright,ref downleft,ref downright} = self.nodes {
			let r = upleft.all_objects_ids();
			for id in r {
				result.push(id);
			}

			let r = upright.all_objects_ids();
			for id in r {
				result.push(id);
			}

			let r = downright.all_objects_ids();
			for id in r {
				result.push(id);
			}

			let r = downleft.all_objects_ids();
			for id in r {
				result.push(id);
			}
		}
		result
	}

	/// Return a Vec of Id of all
	/// the objects that can collide with it
	pub fn query<T:Localisable>(&self, obj: &T) -> Vec<usize> {
		if let FixedQuadtreeBatch::Nil = self.nodes {
			return self.objects_ids();
		}

		let mut ids = self.objects_ids();
		let mut other_ids: Vec<usize> = vec![];

		if let FixedQuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {

			let x = self.x + self.width/2.;
			let y = self.y + self.height/2.;

			match quadrant!(x,y,obj) {
				Quadrant::Upright => { other_ids = upright.query(obj); },
				Quadrant::Upleft => { other_ids = upleft.query(obj); },
				Quadrant::Downright => { other_ids = downright.query(obj); },
				Quadrant::Downleft => { other_ids = downleft.query(obj); },
				Quadrant::Nil => { 
					let r = upright.all_objects_ids();
					for i in r {
						other_ids.push(i);
					}

					let r = upleft.all_objects_ids();
					for i in r {
						other_ids.push(i);
					}

					let r = downright.all_objects_ids();
					for i in r {
						other_ids.push(i);
					}

					let r = downleft.all_objects_ids();
					for i in r {
						other_ids.push(i);
					}
				},
			}

		}

		for id in other_ids {
			ids.push(id);
		}
		ids
	}

	/// insert an object in the fixed quadtree,
	pub fn insert<T:Localisable+Identifiable>(&mut self, obj: &T) {
		if let FixedQuadtreeBatch::Nil = self.nodes {
			self.ids.push(obj.id());
		}
		if let FixedQuadtreeBatch::Cons { ref mut upleft, ref mut upright, ref mut downright, ref mut downleft } = self.nodes {

			let x = self.x + self.width/2.;
			let y = self.y + self.height/2.;

			match quadrant!(x,y,obj) {
				Quadrant::Upright => { upright.insert(obj); },
				Quadrant::Upleft => { upleft.insert(obj); },
				Quadrant::Downright => { downright.insert(obj); },
				Quadrant::Downleft => { downleft.insert(obj); },
				Quadrant::Nil => { self.ids.push(obj.id()); },
			}
		}
	}

	pub fn render_debug(&self, args: &RenderArgs, camera: &Camera, gl: &mut GlGraphics) {
		use graphics::Transformed;
		use graphics::line::{ 
			Line as LineDrawer, 
			Shape as LineShape,
		};
		use graphics::types::Line;
		use graphics::default_draw_state;

		if let FixedQuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {
			const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.5]; 

			let line_drawer = LineDrawer {
				color: GREEN,
				radius: 1.,
				shape: LineShape::Round,
			};

			let mut lines: Vec<Line> = vec![];

			lines.push([
					   self.x,
					   self.y+self.height/2.,
					   self.x+self.width,
					   self.y+self.height/2.]);

			lines.push([
					   self.x+self.width/2.,
					   self.y,
					   self.x+self.width/2.,
					   self.y+self.height]);

			gl.draw(args.viewport(), |context, gl| {
				let transform = camera.trans(context.transform);

				for line in lines {
					line_drawer.draw(line, default_draw_state(), transform, gl);
				}
			});

			upleft.render_debug(args,camera,gl);
			downleft.render_debug(args,camera,gl);
			upright.render_debug(args,camera,gl);
			downright.render_debug(args,camera,gl);
		}
	}

}

impl fmt::Debug for FixedQuadtree {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,">").unwrap();
		for id in &self.ids {
			write!(f,"{:?}",*id).unwrap();
			write!(f,",").unwrap();
		}

		write!(f,"\n").unwrap();

		if let FixedQuadtreeBatch::Cons{ref upleft,ref upright,ref downleft,ref downright} = self.nodes {
			write!(f,"dl: {:?}",downleft).unwrap();
			write!(f,"dr: {:?}",downright).unwrap();
			write!(f,"ul: {:?}",upleft).unwrap();
			write!(f,"ur: {:?}",upright).unwrap();
		} else {
			write!(f,"nil").unwrap();
		}

		write!(f,"\n")
	}
}
