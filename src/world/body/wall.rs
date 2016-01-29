pub struct Wall;

use super::{ 
    Body, 
    CollisionBehavior,
    BodyType,
};
use std::f64;

impl Wall {
    pub fn new(id: usize, x: i32, y: i32, unit: f64) -> Body {
        Body {
            id: id,
            x: (x as f64)*unit,
            y: (y as f64)*unit,
            width2: unit/2.,
            height2: unit/2.,
            weight: f64::MAX,
            velocity: 0.,
            angle: 0.,
            mask: !0,
            group: 1,
            collision_behavior: CollisionBehavior::Stop,
            body_type: BodyType::Wall,
        }
    }
}
