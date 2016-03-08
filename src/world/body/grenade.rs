use rand;
use rand::distributions::{IndependentSample, Range};
use world::batch::Batch;
use std::cell::RefCell;
use super::{
    Body,
    BodyTrait,
    CollisionBehavior,
    BodyType,
};
use frame_manager::{
    color,
    FrameManager,
};
use effect_manager::{EffectManager, Line, Effect};

pub const VELOCITY: f64 = 10.;
pub const TIME_TO_STOP: f64 = 0.8;
pub const TIME_TO_EXPLODE: f64 = TIME_TO_STOP + 1.;
pub const WIDTH: f64 = 1.;
pub const HEIGHT: f64 = 1.;
pub const DAMAGE: f64 = 10.;
pub const WEIGHT: f64 = 1.;
pub const GROUP: u32 = super::GRENADE_GROUP;
pub const MASK: u32 = !0;
pub const NUMBER_OF_SPATTERS: u32 = 32;
pub const SPATTER_MAX_RADIUS: f64 = 10.;

pub struct Grenade {
    body: Body,
    timer: f64,
    alive: bool,
}

impl Grenade {
    pub fn new(id: usize, x: f64, y: f64, angle: f64) -> Grenade {
        Grenade {
            body: Body {
                id: id,
                x: x,
                y: y,
                width: WIDTH,
                height: HEIGHT,
                weight: WEIGHT,
                velocity: VELOCITY,
                angle: angle,
                mask: MASK,
                group: GROUP,
                collision_behavior: CollisionBehavior::Random,
                body_type: BodyType::Grenade,
            },
            alive: true,
            timer: 0.,
        }
    }

    pub fn render(&mut self, frame_manager: &mut FrameManager) {
        self.body.render(color::RED,frame_manager);
    }
}

pub trait GrenadeManager {
    fn update(&self, dt: f64, batch: &Batch, effect_manager: &mut EffectManager);
}

impl GrenadeManager for RefCell<Grenade> {
    fn update(&self, dt: f64, batch: &Batch, effect_manager: &mut EffectManager) {
        use std::f64::consts::PI;

        self.borrow_mut().timer += dt;
        let alive = self.borrow().alive;
        let timer = self.borrow().timer;
        if alive {
            if timer >= TIME_TO_EXPLODE {
                let x = self.borrow().x();
                let y = self.borrow().y();
                let angle_range = Range::new(0.,PI*2.);
                let length_range = Range::new(0.,SPATTER_MAX_RADIUS);
                let mut rng = rand::thread_rng();
                let id = self.borrow().id();

                let mut splatters = vec!();
                for _ in 0..NUMBER_OF_SPATTERS {
                    let angle = angle_range.ind_sample(&mut rng);
                    let mut length = length_range.ind_sample(&mut rng);
                    batch.raycast(x,y,angle,length, &mut |body,min,_| {
                        if body.id() != id && body.body_type() != BodyType::Armory {
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
                self.borrow_mut().set_velocity(0.);
            }
        }
        self.borrow_mut().body.update(dt);
    }
}

impl BodyTrait for Grenade {
    delegate!{
        body:
            id() -> usize,
            body_type() -> BodyType,
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
    }

    fn dead(&self) -> bool {
        !self.alive
    }

    fn on_collision(&mut self, _: &mut BodyTrait) {
        self.timer = TIME_TO_EXPLODE;
    }

    fn damage(&mut self, _: f64) {
        self.timer = TIME_TO_EXPLODE;
    }
}

