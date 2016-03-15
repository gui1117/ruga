use world::body::{Location, CollisionBehavior, PhysicType, Body};
use world::{World, Entity, EntityCell};
use std::cell::{RefCell, Ref, RefMut};
use frame_manager::{color, FrameManager};
use effect_manager::EffectManager;
use super::group;
use utils;

pub const LIFE: f64 = 1.;
pub const WIDTH: f64 = 1.;
pub const HEIGHT: f64 = 1.;
pub const WEIGHT: f64 = 1.;
pub const VELOCITY: f64 = 55.;
pub const MASK: u64 = !0;
pub const GROUP: u64 = super::group::SNAKE;
pub const COLLISION_BEHAVIOR: CollisionBehavior = CollisionBehavior::Persist;
pub const PHYSIC_TYPE: PhysicType = PhysicType::Dynamic;
pub const VISIBLE_RADIUS: f64 = 50.;
pub const DAMAGE: f64 = 1.;
pub const DECISION_TIME: f64 = 0.2;

pub struct Snake {
    body: Body,
    last_decision: f64,
}

impl Snake {
    pub fn new(x: f64, y: f64, angle: f64) -> Snake {
        Snake {
            body: Body {
                id: 0,
                x: x,
                y: y,
                life: LIFE,
                width: WIDTH,
                height: HEIGHT,
                weight: WEIGHT,
                velocity: VELOCITY,
                angle: angle,
                mask: MASK,
                items: Vec::new(),
                group: GROUP,
                collision_behavior: COLLISION_BEHAVIOR,
                physic_type: PHYSIC_TYPE,
            },
            last_decision: 0.,
        }
    }
}

impl EntityCell for RefCell<Snake> {
    fn borrow(&self) -> Ref<Entity> {
        (self as &RefCell<Entity>).borrow()
    }
    fn borrow_mut(&self) -> RefMut<Entity> {
        (self as &RefCell<Entity>).borrow_mut()
    }
    fn update(&self, dt: f64, world: &World, effect_manager: &mut EffectManager) {
    }
}

impl Entity for Snake {
    fn body(&self) -> &Body {
        &self.body
    }
    fn mut_body(&mut self) -> &mut Body {
        &mut self.body
    }
    fn render(&self, frame_manager: &mut FrameManager) {
        self.body.render(color::RED,frame_manager);
    }
    fn on_collision(&mut self, other: &mut Entity) {
        other.mut_body().damage(DAMAGE);
    }
}

