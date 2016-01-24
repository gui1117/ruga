pub struct Wall;

use super::{ Body, CollisionBehavior };
use std::f64;

impl Wall {
    pub fn new(id: usize, x: f64, y: f64, width: f64, height: f64) -> Body {
        Body {
            id: id,
            x: x,
            y: y,
            width2: width/2.,
            height2: height/2.,
            weight: f64::MAX,
            velocity: 0.,
            angle: 0.,
            mask: !0,
            group: 1,
            collision_behavior: CollisionBehavior::Stop,
        }
    }
}
