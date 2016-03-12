use world::{World, Entity, EntityCell};
use world::body::{CollisionBehavior, PhysicType, Body, Item};
use frame_manager::{FrameManager, color};
use effect_manager::EffectManager;
use std::f64;
use std::cell::{RefCell, Ref, RefMut};

pub struct Armory {
    body: Body,
}

impl Armory {
    pub fn new(x: f64, y: f64, item: Item) -> Armory {
        Armory {
            body: Body {
                id: 0,
                x: x,
                y: y,
                width: 4.,
                height: 4.,
                weight: f64::MAX,
                life: f64::MAX,
                velocity: 0.,
                angle: 0.,
                items: vec!(item),
                mask: super::group::CHARACTER,
                group: super::group::ARMORY,
                collision_behavior: CollisionBehavior::Stop,
                physic_type: PhysicType::Kinetic,
            }
        }
    }
}

impl EntityCell for RefCell<Armory> {
    fn borrow(&self) -> Ref<Entity> {
        (self as &RefCell<Entity>).borrow()
    }
    fn borrow_mut(&self) -> RefMut<Entity> {
        (self as &RefCell<Entity>).borrow_mut()
    }
    fn update(&self, _dt: f64, _world: &World, _effect_manager: &mut EffectManager) {
        let mut this = self.borrow_mut();
        if this.body.items.len() == 0 {
            this.body.life = 0.;
        }
    }
}

impl Entity for Armory {
    fn body(&self) -> &Body {
        &self.body
    }
    fn mut_body(&mut self) -> &mut Body {
        &mut self.body
    }
    fn render(&self, frame_manager: &mut FrameManager) {
        if self.body.items.len() > 0 {
            self.body.render(color::RED, frame_manager);
        }
    }
}
