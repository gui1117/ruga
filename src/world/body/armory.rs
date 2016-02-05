pub struct Armory;

use super::{ 
    Body, 
    CollisionBehavior,
    BodyType,
};
use std::f64;

impl Armory {
    pub fn new(id: usize, x: i32, y: i32, unit: f64) -> Body {
        Body {
            id: id,
            x: (x as f64 + 0.5)*unit,
            y: (y as f64 + 0.5)*unit,
            width: unit,
            height: unit,
            weight: f64::MAX,
            velocity: 0.,
            angle: 0.,
            mask: 0,
            group: 128,
            collision_behavior: CollisionBehavior::Stop,
            body_type: BodyType::Armory,
        }
    }
}
