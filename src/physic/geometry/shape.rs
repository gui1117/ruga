//use world::quadtree::Localisable;
use super::primitive::{ Point, Rectangle };
use std::f64;
use std::f64::consts::PI;

/// the minimanl difference of two angle when computing shape normal
pub const DIFF_NORMAL: f64 = 0.1;

#[derive(Clone)]
pub struct Shape {
    /// x position of the center
    pub x: f64,
    /// y position of the center
    pub y: f64,
    /// angle
    pub angle: f64,
    /// edges relative to the center and the angle
    pub edges: Vec<Point>,
    /// angle of the normals of each edges
	pub normals: Vec<f64>,
    /// smallest box surrounding 
    bounding_box: Rectangle,
}

#[derive(Clone,Debug)]
pub enum ShapeError {
    NotConvex,
    LessThan3EdgesNotAligned,
}

fn delete_aligned(edges: &mut Vec<Point>) {
    let mut i = 0;
    while edges.len() > 3 && i < edges.len() {
        let len = edges.len();
        match Point::angle(&edges[i%len], &edges[(i+1)%len], &edges[(i+2)%len]) {
            0. => { edges.remove((i+1)%len); },
            _ => { i += 1; },

        }
    }
}

fn is_convex(edges: &Vec<Point>) -> bool {
    let witness = Point::angle(&edges[0],&edges[1],&edges[2]) > 0.;
    for i in 1..edges.len()-1 {
        let len = edges.len();
        if (Point::angle(&edges[i],&edges[(i+1)%len],&edges[(i+2)%len]) > 0.) != witness {
            return false;
        }
    }
    return true;
}

fn compute_normals(edges: &Vec<Point>) -> Vec<f64> {
    // compute normals
    let v = Point { 
        x: edges[0].x-edges[edges.len()-1].x, 
        y: edges[0].y-edges[edges.len()-1].y 
    };

    let mut normals = vec![v.angle_0x()-f64::consts::PI/2.];
    for i in 0..edges.len()-1 {
        let j = i+1;
        let v = Point { x: edges[j].x-edges[i].x, y: edges[j].y-edges[i].y };
        normals.push(v.angle_0x()-f64::consts::PI/2.);
    }

    // delete normal that are equal
    let mut i = 0;
    while i < normals.len()-1 {
        let mut j = i+1;
        while j < normals.len() {
            if ((normals[i] - normals[j]) % PI).abs() < DIFF_NORMAL {
                normals.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }

    normals
}

impl Shape {
    /// create a new Shape, it panics if the shape isn't a convex polygon.
    /// Also it cannot be a line
    pub fn new(x: f64, y: f64, angle: f64, edges: Vec<Point>) -> Result<Shape,ShapeError> {
        let mut edges = edges;

        delete_aligned(&mut edges);

        if edges.len() < 3 {
            return Err(ShapeError::LessThan3EdgesNotAligned);
        }

        if !is_convex(&edges) {
            return Err(ShapeError::NotConvex);
        }

        let normals = compute_normals(&edges);

        let mut shape = Shape { 
            x: x,
            y: y,
            angle: angle,
            edges: edges,
            normals: normals,
            bounding_box: Rectangle {
                downleft: Point {x:0.,y:0.},
                width: 0.,
                height: 0.,
            }
        };

        let (min_x,max_x) = shape.min_max(0.);
        let (min_y,max_y) = shape.min_max(PI);

        shape.bounding_box.downleft.x = min_x;
        shape.bounding_box.downleft.y = min_y;
        shape.bounding_box.width = max_x-min_x;
        shape.bounding_box.height = max_y-min_y;

        Ok(shape)
    }

	/// to have the minimal and maximal projection on to the axe of
	/// an angle
    ///
    /// return (min,max)
	pub fn min_max(&self, dir_angle: f64) -> (f64,f64) {
		let rel_dir = Point { 
            x:f64::cos(dir_angle-self.angle), 
            y:f64::sin(dir_angle-self.angle) 
        };

		let mut min = f64::INFINITY;
		let mut max = f64::NEG_INFINITY;

		for edge in &self.edges {
			let cross = edge.x*rel_dir.x + edge.y*rel_dir.y;
			if cross > max { max = cross; }
			if cross < min { min = cross; }
		}

		let dir = Point { 
            x:f64::cos(dir_angle), 
            y:f64::sin(dir_angle) 
        };

		let cross = self.x*dir.x + self.y*dir.y;

		(min+cross,max+cross)
	}

	/// compute if two shape each relative to a point x and y and also with an angle
	/// are overlapping or not
	/// it return a boolean for overlapping or not and the angle of minimal overlap and the length
	/// of overlapping 
	/// the vector returned is the vector you to move b so it doesn't overlap anymore
	pub fn overlap(a: &Shape, b: &Shape) -> (bool,f64,f64) {

		let mut min_angle = 0.;
		let mut min_length = f64::INFINITY;

		for n in &a.normals {
			let (a_min,a_max) = a.min_max(*n + a.angle);
			let (b_min,b_max) = b.min_max(*n + a.angle);
			
			let length = (a_max-b_min).min(b_max-a_min);

			if length <= 0. {
				return (false,0.,0.);
			} else if min_length > length {
				min_length = length;
				
				if length == a_max-b_min {
					min_angle = *n + a.angle;
				} else {
					min_angle = *n + a.angle + PI;
				}
			}
		}

		for n in &b.normals {
			let (a_min,a_max) = a.min_max(*n + b.angle);
			let (b_min,b_max) = b.min_max(*n + b.angle);

			let length = (a_max-b_min).min(b_max-a_min);

			if length <= 0. {
				return (false,0.,0.);
			} else if min_length > length {
				min_length = length;

				if length == a_max-b_min {
					min_angle = *n + b.angle;
				} else {
					min_angle = *n + b.angle + PI;
				}
			}
		}

		(true,min_length,min_angle)
	}
}



