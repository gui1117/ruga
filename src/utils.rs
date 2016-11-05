use specs::Entity;
use collider::HitboxId;
use collider::Hitbox;
use collider::geom::{Shape, PlacedShape, vec2};

use std::mem;

pub trait AsColliderId {
    fn aci(self) -> HitboxId;
}

pub trait AsSpecsId {
    fn asi(self) -> Entity;
}

impl AsSpecsId for HitboxId {
    fn asi(self) -> Entity {
        unsafe { mem::transmute(self) }
    }
}

impl AsColliderId for Entity {
    fn aci(self) -> HitboxId {
        unsafe { mem::transmute(self) }
    }
}

pub fn circle_hitbox(x: f32, y: f32, radius: f32) -> Hitbox {
    Hitbox::new(PlacedShape::new(vec2(x as f64, y as f64), Shape::new_circle(radius as f64 * 2.0)))
}

pub fn rect_hitbox(x: f32, y: f32, width: f32, height: f32) -> Hitbox {
    Hitbox::new(PlacedShape::new(vec2(x as f64, y as f64), Shape::new_rect(vec2(width as f64, height as f64))))
}
