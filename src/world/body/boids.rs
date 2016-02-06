use world::batch::Batch;
use world::spatial_hashing::Location;
use std::cell::RefCell;
use std::f64::consts::PI;
use util::minus_pi_pi;

use super::{ 
    Body, 
    BodyTrait, 
    CollisionBehavior,
    BodyType,
};

pub struct Boid {
    body: Body,
    life: f64,
}

pub const LIFE: f64 = 1.;
pub const WIDTH: f64 = 1.;
pub const HEIGHT: f64 = 1.;
pub const WEIGHT: f64 = 1.;
pub const MASK: u32 = !0;
pub const GROUP: u32 = super::BOID_GROUP;
pub const DAMAGE: f64 = 10.;
pub const VELOCITY: f64 = 50.;

pub const COHESION_RADIUS: f64 = 40.;
pub const COHESION_MAX_DELTA_ANGLE: f64 = PI;
pub const COHESION_FACTOR: f64 = 5.;

impl Boid {
    pub fn new(id: usize, x: f64, y: f64, angle: f64) -> Boid {
        Boid {
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
                body_type: BodyType::Boid,
            },
            life: LIFE,
        }
    }

    pub fn render_debug(&self, lines: &mut Vec<[f64;4]>) {
        self.body.render_debug(lines);
    }
}

pub trait BoidManager {
    fn update(&self,dt:f64,batch: &Batch);
}

impl BoidManager for RefCell<Boid> {
    fn update(&self, dt: f64, batch: &Batch) {
        let mut counter = 0;
        let mut sum = 0.;

        {
            let (location,id,angle) = {
                let this = self.borrow();
                (Location {
                    up: this.body.y() + COHESION_RADIUS,
                    down: this.body.y() - COHESION_RADIUS,
                    right: this.body.x() + COHESION_RADIUS,
                    left: this.body.x() - COHESION_RADIUS,
                }, this.id(), this.angle())
            };

            let mut callback = |body: &mut BodyTrait| {
                if body.body_type() == BodyType::Boid && body.id() != id {
                    let delta_angle = minus_pi_pi(body.angle() - angle);
                    if delta_angle.abs() < COHESION_MAX_DELTA_ANGLE {
                        counter += 1;
                        sum += delta_angle;
                    }
                }
            };

            batch.apply_locally(&location,&mut callback);
        }

        let mut this = self.borrow_mut();
        if counter > 0 {
            let a = this.angle() - dt*COHESION_FACTOR*sum/(counter as f64);
            this.set_angle(a);
        }
        this.body.update(dt);
    }
}

impl BodyTrait for Boid {
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
        self.life <= 0.
    }

    fn on_collision(&mut self, other: &mut BodyTrait) {
        if other.body_type() != BodyType::Boid {
            other.damage(DAMAGE);
        }
    }

    fn damage(&mut self, damage: f64) {
        self.life -= damage;
    }
}
