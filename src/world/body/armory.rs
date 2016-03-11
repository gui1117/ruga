use super::{
    Body,
    CollisionBehavior,
    BodyType,
    BodyTrait,
    PhysicType,
};
use frame_manager::{FrameManager, color};
use std::f64;

pub struct Armory {
    body: Body,
}

impl Armory {
    pub fn new(id: usize, x: i32, y: i32, unit: f64) -> Armory {
        Armory {
            body: Body {
                id: id,
                x: (x as f64 + 0.5)*unit,
                y: (y as f64 + 0.5)*unit,
                width: unit,
                height: unit,
                weight: f64::MAX,
                velocity: 0.,
                angle: 0.,
                mask: super::group::CHARACTER,
                group: super::group::ARMORY,
                collision_behavior: CollisionBehavior::Stop,
                body_type: BodyType::Armory,
                physic_type: PhysicType::Kinetic,
            }
        }
    }
    pub fn render(&self, frame_manager: &mut FrameManager) {
        self.body.render(color::RED, frame_manager);
    }
}

impl BodyTrait for Armory {
    delegate!{
        body:
            id() -> usize,
            dead() -> bool,
            body_type() -> BodyType,
            mut damage(d: f64) -> (),
            width() -> f64,
            height() -> f64,
            x() -> f64,
            mut set_x(x: f64) -> (),
            y() -> f64,
            mut set_y(y: f64) -> (),
            weight() -> f64,
            velocity() -> f64,
            mut set_velocity(v: f64) -> (),
            angle() -> f64,
            mut set_angle(a: f64) -> (),
            mask() -> u32,
            group() -> u32,
            collision_behavior() -> CollisionBehavior,
            physic_type() -> PhysicType,
    }

    fn on_collision(&mut self, other: &mut BodyTrait) {
        //TODO
    }
}
