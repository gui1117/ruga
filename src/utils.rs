/// return the angle in ]-PI,PI]
#[inline]
pub fn minus_pi_pi(a: f32) -> f32 {
    use ::std::f32::consts::PI;
    use ::std::ops::Rem;

    if a.abs() < PI {
        a
    } else if a == PI {
        a
    } else {
        let a = a.rem(2. * PI);
        if a > PI {
            a - 2. * PI
        } else if a <= -PI {
            a + 2. * PI
        } else {
            a
        }
    }
}

pub mod math {
    use rusttype::Vector;

    #[inline]
    pub fn angle_into_vector(angle: f32) -> Vector<f32> {
        Vector {
            x: angle.cos(),
            y: angle.sin(),
        }
    }
    #[inline]
    pub fn into_vector(p: [f32; 2]) -> Vector<f32> {
        Vector {
            x: p[0],
            y: p[1],
        }
    }
    #[inline]
    pub fn angle(p: [f32; 2]) -> f32 {
        p[1].atan2(p[0])
    }
    #[inline]
    pub fn norm(p: [f32; 2]) -> f32 {
        (p[0].powi(2) + p[1].powi(2)).sqrt()
    }
    #[inline]
    pub fn mul(k: f32, p: [f32; 2]) -> [f32; 2] {
        [p[0]*k, p[1]*k]
    }
    #[inline]
    pub fn normalize(p: [f32; 2]) -> [f32; 2] {
        mul(1./norm(p), p)
    }
    #[inline]
    pub fn add(p1: [f32; 2], p2: [f32; 2]) -> [f32; 2] {
        [p1[0]+p2[0], p1[1]+p2[1]]
    }
    #[inline]
    pub fn sub(p1: [f32; 2], p2: [f32; 2]) -> [f32; 2] {
        [p1[0]-p2[0], p1[1]-p2[1]]
    }
}

macro_rules! infer_type {
    () => {::hlua::function0};
    ($t1:tt) => {::hlua::function1};
    ($t1:tt $t2:tt) => {::hlua::function2};
    ($t1:tt $t2:tt $t3:tt) => {::hlua::function3};
    ($t1:tt $t2:tt $t3:tt $t4:tt) => {::hlua::function4};
    ($t1:tt $t2:tt $t3:tt $t4:tt $t5:tt) => {::hlua::function5};
    ($t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt) => {::hlua::function6};
    ($t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt) => {::hlua::function7};
    ($t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt) => {::hlua::function8};
    ($t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $t6:tt $t7:tt $t8:tt $t9:tt) => {::hlua::function9};
}
