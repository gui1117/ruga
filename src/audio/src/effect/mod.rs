//! this module allow to play short and persistent sound effects
//!
//! be careful that `set_volume`, `set_listener`, `set_distance_model`
//! only affect future short sound effects

pub mod short;
pub mod persistent;

use super::RAW_STATE;

/// stop all short effects and mute all persistent effects
pub fn stop() {
    unimplemented!();
}

#[doc(hidden)]
pub fn update_volume() {
    short::update_volume();
    persistent::update_volume_for_all();
}

/// set the volume of sound effects
/// take effect for future sounds effects only
pub fn set_volume(v: f32) {
    let mut state = unsafe { (*RAW_STATE).write().unwrap() };
    state.effect_volume = v;
    update_volume();
}

/// return the volume of sound effects
pub fn volume() -> f32 {
    let state = unsafe { (*RAW_STATE).read().unwrap() };
    state.effect_volume
}

/// set the position of the listener
pub fn set_listener(pos: [f32;3]) {
    let mut state = unsafe { (*RAW_STATE).write().unwrap() };
    state.listener = pos;
}

/// return the position of the listener
pub fn listener() -> [f32;3] {
    let state = unsafe { (*RAW_STATE).read().unwrap() };
    state.listener
}

/// set the distance model
pub fn set_distance_model(d: DistanceModel) {
    let mut state = unsafe { (*RAW_STATE).write().unwrap() };
    state.distance_model = d;
}

/// distance model, used to compute sound effects volumes.
#[derive(Clone,Debug,PartialEq,RustcDecodable,RustcEncodable)]
pub enum DistanceModel {
    /// if d <= a then 1
    ///
    /// if a <= d <= b then 1-((d-a)/(b-a))
    ///
    /// if d >= b then 0
    Linear(f32,f32),
    /// if d <= a then 1
    ///
    /// if a <= d <= b then (1-((d-a)/(b-a)))^2
    ///
    /// if d >= b then 0
    Pow2(f32,f32),
}

impl DistanceModel {
    fn distance(&self, pos: [f32;3], listener: [f32;3]) -> f32 {
        let d = pos.iter()
            .zip(&listener)
            .map(|(a,b)| (a-b).powi(2))
            .fold(0.,|sum,i| sum+i)
            .sqrt();

        match *self {
            DistanceModel::Linear(a,b) => {
                if d <= a {
                    1.
                } else if d <= b {
                    1. - ((d-a)/(b-a))
                } else {
                    0.
                }
            }
            DistanceModel::Pow2(a,b) => {
                if d <= a {
                    1.
                } else if d <= b {
                    (1. - ((d-a)/(b-a))).powi(2)
                } else {
                    0.
                }
            }
        }
    }
}

#[test]
fn test_distance() {
    let origin = [0.,0.,0.];
    let d = DistanceModel::Linear(10.,110.);
    assert_eq!(d.distance(origin,origin), 1.);
    assert_eq!(d.distance(origin,[10.,0.,0.]), 1.);
    assert_eq!(d.distance(origin,[60.,0.,0.]), 0.5);
    assert!(d.distance(origin,[100.,0.,0.]) - 0.1 < 0.00001);
    assert_eq!(d.distance(origin,[150.,0.,0.]), 0.);
}
