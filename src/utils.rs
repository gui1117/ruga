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
