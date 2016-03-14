use world::body::{Location, CollisionBehavior, PhysicType, Body};
use world::{World, Entity, EntityCell};
use std::cell::{RefCell, Ref, RefMut};
use frame_manager::{color, FrameManager};
use effect_manager::EffectManager;
use super::group;

pub const LIFE: f64 = 1.;
pub const WIDTH: f64 = 1.;
pub const HEIGHT: f64 = 1.;
pub const WEIGHT: f64 = 1.;
pub const VELOCITY: f64 = 55.;
pub const MASK: u64 = !0;
pub const GROUP: u64 = super::group::SPIDER;
pub const COLLISION_BEHAVIOR: CollisionBehavior = CollisionBehavior::Persist;
pub const PHYSIC_TYPE: PhysicType = PhysicType::Dynamic;
pub const VISIBLE_RADIUS: f64 = 50.;
pub const DAMAGE: f64 = 1.;

pub struct Spider {
    body: Body,
}

impl Spider {
    pub fn new(x: f64, y: f64, angle: f64) -> Spider {
        Spider {
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
        }
    }
}

impl EntityCell for RefCell<Spider> {
    fn borrow(&self) -> Ref<Entity> {
        (self as &RefCell<Entity>).borrow()
    }
    fn borrow_mut(&self) -> RefMut<Entity> {
        (self as &RefCell<Entity>).borrow_mut()
    }
    fn update(&self, dt: f64, world: &World, effect_manager: &mut EffectManager) {
        let x = self.borrow().body.x;
        let y = self.borrow().body.y;
        let loc = Location {
            up: y + VISIBLE_RADIUS,
            down: y - VISIBLE_RADIUS,
            left: x - VISIBLE_RADIUS,
            right: x + VISIBLE_RADIUS,
        };
        let mut prey_pos = None;
        world.apply_locally(group::CHARACTER, &loc, &mut |entity: &mut Entity| {
            prey_pos = Some((entity.body().x,entity.body().y));
        });
        if let Some((prey_x,prey_y)) = prey_pos {
            // TODO
            // if let Some(angle) = world.get_path_angle(x,y,prey_x,prey_y) {
            //     self.borrow_mut().body.angle = angle;
            //     self.borrow_mut().body.velocity = VELOCITY;
            // } else {
            //     self.borrow_mut().body.velocity = 0.;
            // }
        }
    }
}

impl Entity for Spider {
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

