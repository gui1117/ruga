use rand;
use rand::distributions::{IndependentSample, Range};
use std::cell::{RefCell, Ref, RefMut};
use world::body::{CollisionBehavior, PhysicType, Body};
use world::{World, Entity, EntityCell};
use frame_manager::{
    color,
    FrameManager,
};
use effect_manager::{EffectManager, Line, Effect};
use super::group;

pub const VELOCITY: f64 = 10.;
pub const TIME_TO_STOP: f64 = 0.8;
pub const TIME_TO_EXPLODE: f64 = TIME_TO_STOP + 1.;
pub const LIFE: f64 = 1.;
pub const WIDTH: f64 = 1.;
pub const HEIGHT: f64 = 1.;
pub const DAMAGE: f64 = 10.;
pub const WEIGHT: f64 = 1.;
pub const GROUP: u64 = super::group::GRENADE;
pub const MASK: u64 = !0;
pub const NUMBER_OF_SPATTERS: u64 = 32;
pub const SPATTER_MAX_RADIUS: f64 = 10.;
pub const COLLISION_BEHAVIOR: CollisionBehavior = CollisionBehavior::Bounce;
pub const PHYSIC_TYPE: PhysicType = PhysicType::Dynamic;

pub struct Grenade {
    body: Body,
    timer: f64,
    alive: bool,
}

impl Grenade {
    pub fn new(x: f64, y: f64, angle: f64) -> Grenade {
        Grenade {
            body: Body {
                items: Vec::new(),
                id: 0,
                life: LIFE,
                x: x,
                y: y,
                width: WIDTH,
                height: HEIGHT,
                weight: WEIGHT,
                velocity: VELOCITY,
                angle: angle,
                mask: MASK,
                group: GROUP,
                collision_behavior: COLLISION_BEHAVIOR,
                physic_type: PHYSIC_TYPE,
            },
            alive: true,
            timer: 0.,
        }
    }

    pub fn render(&mut self, frame_manager: &mut FrameManager) {
        self.body.render(color::RED,frame_manager);
    }
}

impl EntityCell for RefCell<Grenade> {
    fn borrow(&self) -> Ref<Entity> {
        (self as &RefCell<Entity>).borrow()
    }
    fn borrow_mut(&self) -> RefMut<Entity> {
        (self as &RefCell<Entity>).borrow_mut()
    }
    fn update(&self, dt: f64, world: &World, effect_manager: &mut EffectManager) {
        use std::f64::consts::PI;

        self.borrow_mut().timer += dt;
        let alive = self.borrow().alive;
        let timer = self.borrow().timer;
        if alive {
            if timer >= TIME_TO_EXPLODE {
                let x = self.borrow().body.x;
                let y = self.borrow().body.y;
                let id = self.borrow().body.id;
                let angle_range = Range::new(0.,PI*2.);
                let length_range = Range::new(0.,SPATTER_MAX_RADIUS);
                let mut rng = rand::thread_rng();

                let mut splatters = vec!();
                for _ in 0..NUMBER_OF_SPATTERS {
                    let angle = angle_range.ind_sample(&mut rng);
                    let mut length = length_range.ind_sample(&mut rng);
                    world.raycast(!group::ARMORY,x,y,angle,length, &mut |entity,min,_| {
                        let body = entity.mut_body();
                        if body.id != id {
                            body.damage(DAMAGE);
                            length = min;
                            true
                        } else {
                            false
                        }
                    });
                    splatters.push(Line::new(x,y,angle,length));
                }
                effect_manager.add(Effect::GrenadeExplosion(splatters));

                self.borrow_mut().alive = false;
            } else if timer >= TIME_TO_STOP {
                self.borrow_mut().body.velocity = 0.;
            }
        }
        self.borrow_mut().body.update(dt);
    }
}

impl Entity for Grenade {
    fn body(&self) -> &Body {
        &self.body
    }
    fn mut_body(&mut self) -> &mut Body {
        &mut self.body
    }
    fn render(&self, frame_manager: &mut FrameManager) {
        self.body.render(color::RED,frame_manager);
    }
    fn on_collision(&mut self, _: &mut Entity) {
        self.timer = TIME_TO_EXPLODE;
    }
}

