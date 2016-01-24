pub struct Monster;

use super::{ 
    Body, 
    CollisionBehavior,
    BodyType,
};

impl Monster {
    pub fn new(id: usize, x: f64, y: f64, angle: f64) -> Body {
        Body {
            id: id,
            x: x,
            y: y,
            width2: 5.,
            height2: 5.,
            weight: 1.,
            velocity: 0.,
            angle: angle,
            mask: !0,
            group: 2,
            collision_behavior: CollisionBehavior::Persist,
            body_type: BodyType::Monster,
        }
    }
}
