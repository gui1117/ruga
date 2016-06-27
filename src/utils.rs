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

pub trait Into3D {
    fn into_3d(&self) -> [f64;3];
}

impl Into3D for [f32;2] {
    fn into_3d(&self) -> [f64;3] {
        [self[0] as f64,self[1] as f64,0f64]
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

