use std::fmt;

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
