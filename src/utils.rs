#[derive(Debug,Clone,Copy,PartialEq)]
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

    pub fn opposite(&self) -> Direction {
        match self {
            &Direction::Up => Direction::Down,
            &Direction::Down => Direction::Up,
            &Direction::Left => Direction::Right,
            &Direction::Right => Direction::Left,
        }
    }
}

///return the angle in ]-PI,PI]
pub fn minus_pi_pi(a: f32) -> f32 {
    use std::f32::consts::PI;
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
    use std::f32::consts::PI;
    assert_eq!(minus_pi_pi(PI),PI);
    assert_eq!(minus_pi_pi(-PI),PI);
    assert_eq!(minus_pi_pi(3.*PI),PI);
    assert_eq!(minus_pi_pi(3.*PI),PI);
}

//TODO replace by action: up, down ...
pub mod key {
    pub const Z:      u8 = 25;
    pub const Q:      u8 = 38;
    pub const S:      u8 = 39;
    pub const D:      u8 = 40;
    pub const E:      u8 = 26;
    pub const R:      u8 = 27;
    pub const T:      u8 = 28;
    pub const Y:      u8 = 29;
    pub const ESCAPE: u8 = 9;
}

