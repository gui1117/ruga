///return the angle in ]-PI,PI]
#[inline]
pub fn minus_pi_pi(a: f32) -> f32 {
    use ::std::f32::consts::PI;
    use ::std::ops::Rem;

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

