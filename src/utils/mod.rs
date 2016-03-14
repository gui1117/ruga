use std::fmt;
use std::collections::{BTreeSet, HashSet, BTreeMap};
use std::ops::{Add, Sub};
use std::u32;

#[derive(Clone,Copy,PartialEq)]
pub enum Direction {
	Left,
	Right,
	Up,
	Down,
}

impl Direction {
	pub fn perpendicular(&self, other: &Direction) -> bool {
		match self {
			&Direction::Up | &Direction::Down => {
				match other {
					&Direction::Right | &Direction::Left => true,
					_ => false,
				}
			},

			&Direction::Right | &Direction::Left => {
				match other {
					&Direction::Up | &Direction::Down => true,
					_ => false,
				}
			},
		}
	}

    pub fn to_f64(&self) -> f64 {
        use std::f64::consts::*;
        match self {
			&Direction::Up => FRAC_PI_2,
			&Direction::Down => -FRAC_PI_2,
			&Direction::Left => PI,
			&Direction::Right => 0.,
		}
    }

    pub fn opposite(&self) -> Direction {
        match self {
			&Direction::Up => Direction::Down,
			&Direction::Down => Direction::Up,
			&Direction::Left => Direction::Right,
			&Direction::Right => Direction::Left,
		}
    }
}

impl fmt::Debug for Direction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Direction::Up => write!(f,"Up"),
			&Direction::Down => write!(f,"Down"),
			&Direction::Left => write!(f,"Left"),
			&Direction::Right => write!(f,"Right"),
		}
	}
}

///return the angle in ]-PI,PI]
pub fn minus_pi_pi(a: f64) -> f64 {
    use std::f64::consts::PI;
    use std::ops::Rem;

    if a.abs() < PI {
        a
    } else if a == PI {
        a
    } else {
        let a = a.rem(2.*PI);
        if a > PI {
            a - 2.*PI
        } else if a <= -PI {
            a + 2.*PI
        } else {
            a
        }
    }
}

#[test]
fn test_minus_pi_pi() {
    use std::f64::consts::PI;
    assert_eq!(minus_pi_pi(PI),PI);
    assert_eq!(minus_pi_pi(-PI),PI);
    assert_eq!(minus_pi_pi(3.*PI),PI);
    assert_eq!(minus_pi_pi(3.*PI),PI);
}

/// https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode
pub fn get_path_angle(x: f64, y: f64, prey_x: f64, prey_y: f64, vision_radius: f64, unit: f64, wall_map: &HashSet<(i32,i32)>) -> Option<f64> {
    let x = (x/unit).floor() as i32;
    let y = (y/unit).floor() as i32;
    let prey_x = (prey_x/unit).floor() as i32;
    let prey_y = (prey_y/unit).floor() as i32;
    let max_cost = ((vision_radius/unit)*1.5).ceil() as u32;
    a_star_angle([x,y],[prey_x,prey_y],max_cost,wall_map)
}

fn a_star_angle(start: [i32;2], goal: [i32;2], max_cost: u32, wall_map: &HashSet<(i32,i32)>) -> Option<f64> {
    let mut closed_set = BTreeSet::new();
    let mut open_set = BTreeSet::new();
    open_set.insert(start);
    let mut came_from = BTreeMap::new();
    let mut g_score = BTreeMap::new();
    g_score.insert(start,0);
    let mut f_score = BTreeMap::new();
    f_score.insert(start,max_cost);
    while !open_set.is_empty() {
        let current = {
            let (current,_) = open_set.iter().fold((None,u32::MAX), |(lowest_node,lowest_score),element| {
                if let Some(&element_score) = f_score.get(element) {
                    if lowest_score > element_score {
                        (Some(element),element_score)
                    } else {
                        (lowest_node,lowest_score)
                    }
                } else if let None = lowest_node {
                    (Some(element),u32::MAX)
                } else {
                    (lowest_node,lowest_score)
                }
            });
            *current.unwrap()
        };
        open_set.remove(&current);
        if current == goal {
            return Some(reconstruct_and_get_path_angle(came_from,goal));
        }
        closed_set.insert(current);
        for &neighbor in &neighbor_node(current,wall_map) {
            if closed_set.contains(&neighbor) {
                continue;
            }
            let tentative_g_score = g_score[&current] + 1;
            if !open_set.contains(&neighbor) {
                open_set.insert(neighbor);
            } else if tentative_g_score >= g_score[&current] {
                continue;
            }
            came_from.remove(&neighbor);
            came_from.insert(neighbor,current);
            g_score.remove(&neighbor);
            g_score.insert(neighbor,tentative_g_score);
            f_score.remove(&neighbor);
            let heuristic_cost = {
                let x_dist = (neighbor[0]-current[0]).abs();
                let y_dist = (neighbor[1]-current[1]).abs();
                if x_dist > y_dist {
                    x_dist as u32
                } else {
                    y_dist as u32
                }
            };
            f_score.insert(neighbor,tentative_g_score+heuristic_cost);
        }
    }
    None
}

fn reconstruct_and_get_path_angle(came_from: BTreeMap<[i32;2],[i32;2]>, goal: [i32;2]) -> f64 {
    let mut vec = Vec::new();
    let mut current = goal;
    vec.push(current);
    while let Some(&pred) = came_from.get(&current) {
        vec.push(pred);
        current = pred;
    }
    if vec.len() == 1 {
        0.
    } else {
        vec.reverse();
        let start = Point {x: vec[0][0] as f64,y: vec[0][1] as f64};
        let next = Point {x: vec[1][0] as f64,y: vec[1][1] as f64};
        next.sub(start).angle_0x()
    }
}

fn neighbor_node(n: [i32;2],wall_map: &HashSet<(i32,i32)>) -> Vec<[i32;2]> {
    let mut vec = vec!(
        [n[0]+1,n[1]+1],
        [n[0]+0,n[1]+1],
        [n[0]-1,n[1]+1],
        [n[0]+1,n[1]+0],
        [n[0]-1,n[1]+0],
        [n[0]+1,n[1]-1],
        [n[0]+0,n[1]-1],
        [n[0]-1,n[1]-1],
        );
    vec.retain(|index| !wall_map.contains(&(index[0],index[1])));
    vec
}

#[test]
fn test_pathfinding() {
    let start = [0,0];
    let goal = [1,2];
    let mut wall_map = HashSet::new();
    wall_map.insert((0,1));
    wall_map.insert((1,0));
    wall_map.insert((1,1));
    let angle = a_star_angle(start,goal,100,&wall_map).unwrap();
    assert!((angle - 2.3).abs() < 0.1);

    let start = [0,0];
    let goal = [2,1];
    let mut wall_map = HashSet::new();
    wall_map.insert((0,1));
    wall_map.insert((1,0));
    wall_map.insert((1,1));
    let angle = a_star_angle(start,goal,100,&wall_map).unwrap();
    assert!((angle - -0.7).abs() < 0.1);
}

/// simple structure, it represents a coordinate
#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// return the angle between the vector ba and bc.
    /// The angle is in ]-Pi,Pi]
    pub fn angle(a: &Point, b: &Point, c: &Point) -> f64 {
        let u = Point { x: a.x-b.x, y: a.y-b.y };
        let v = Point { x: c.x-b.x, y: c.y-b.y };
        let vectorial_product = u.x*v.y-u.y*v.x;
        let scalar_product = u.x*v.x+u.y*v.y;
        vectorial_product.atan2(scalar_product)
    }

    /// return the angle between 0x and the point ad
    /// a vector
    pub fn angle_0x(&self) -> f64 {
        Self::angle(
            &Point { x: 1., y: 0. },
            &Point { x: 0., y: 0. },
            self)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
