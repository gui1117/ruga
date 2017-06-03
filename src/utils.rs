use gilrs;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    #[inline]
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

    #[inline]
    #[allow(dead_code)]
    pub fn opposite(&self) -> Direction {
        match self {
            &Direction::Up => Direction::Down,
            &Direction::Down => Direction::Up,
            &Direction::Left => Direction::Right,
            &Direction::Right => Direction::Left,
        }
    }
}

///return the angle in ]-PI, PI]
#[inline]
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

pub trait Into3D {
    fn into_3d(&self) -> [f32;3];
}

impl Into3D for [f32;2] {
    #[inline]
    fn into_3d(&self) -> [f32;3] {
        [self[0], self[1], 0f32]
    }
}

#[test]
fn test_minus_pi_pi() {
    use std::f32::consts::PI;
    assert!((minus_pi_pi(PI)-PI).abs() < 0.001);
    assert!((minus_pi_pi(-PI)-PI).abs() < 0.001);
    assert!((minus_pi_pi(3.*PI)-PI).abs() < 0.001);
    assert!((minus_pi_pi(3.*PI)-PI).abs() < 0.001);
}

pub trait HorizontalVerticalAxis {
    fn is_horizontal(&self) -> bool;
    fn is_vertical(&self) -> bool;
}

impl HorizontalVerticalAxis for gilrs::Axis {
    #[inline]
    fn is_horizontal(&self) -> bool {
        use gilrs::Axis::*;
        match *self {
            LeftStickX | RightStickX => true,
            _ => false,
        }
    }
    #[inline]
    fn is_vertical(&self) -> bool {
        use gilrs::Axis::*;
        match *self {
            LeftStickY | RightStickY => true,
            _ => false,
        }
    }
}

#[inline]
pub fn inside_rectangle(loc: [f64;2], rec: [f64;4]) -> bool {
    (loc[0] - rec[0]).abs() < rec[2]/2. && (loc[1]-rec[1]).abs() < rec[3]/2.
}
