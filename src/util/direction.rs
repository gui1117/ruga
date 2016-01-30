use std::fmt; 

#[derive(Clone)]
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
			&Direction::Up => -FRAC_PI_2,
			&Direction::Down => FRAC_PI_2,
			&Direction::Left => PI,
			&Direction::Right => 0.,
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

