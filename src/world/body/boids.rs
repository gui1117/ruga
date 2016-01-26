use viewport::Viewport;
use opengl_graphics::GlGraphics;
use world::{ 
    Camera, 
};
use world::batch::Batch;
use world::spatial_hashing::Location;
use std::rc::Rc;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::ops::Rem;

use super::{ 
    Body, 
    BodyTrait, 
    CollisionBehavior,
    BodyType,
};

pub struct Boid {
    body: Body,
    alive: bool,
    world_batch: Rc<RefCell<Batch>>,
}

pub const WIDTH: f64 = 10.;
pub const HEIGHT: f64 = 10.;
pub const WEIGHT: f64 = 1.;
pub const MASK: u32 = !0;
pub const GROUP: u32 = 4;
pub const DAMAGE: f64 = 10.;
pub const VELOCITY: f64 = 100.;

pub const COHESION_RADIUS: f64 = 50.;
pub const COHESION_MAX_DELTA_ANGLE: f64 = 2.*PI;
pub const COHESION_FACTOR: f64 = 10.0;

impl Boid {
    pub fn new(id: usize, x: f64, y: f64, angle: f64, batch: Rc<RefCell<Batch>>) -> Boid {
        Boid {
            body: Body {
                id: id,
                x: x,
                y: y,
                width2: WIDTH/2.,
                height2: HEIGHT/2.,
                weight: WEIGHT,
                velocity: VELOCITY,
                angle: angle,
                mask: MASK,
                group: GROUP,
                collision_behavior: CollisionBehavior::Bounce,
                body_type: BodyType::Boid,
            },
            alive: true,
            world_batch: batch,
        }
    }
}

impl BodyTrait for RefCell<Boid> {
    delegate!{
        body:
           id() -> usize,
           body_type() -> BodyType,
           width2() -> f64,
           height2() -> f64,
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
           render(viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) -> (),
           render_debug(lines: &mut Vec<[f64;4]>) -> (),
    }

    fn update(&self, dt: f64) {
        let mut counter = 0;
        let mut sum = 0.;
        {
            let location;
            {
                location = Location {
                    up: self.borrow().body.y() + COHESION_RADIUS,
                    down: self.borrow().body.y() - COHESION_RADIUS,
                    right: self.borrow().body.x() + COHESION_RADIUS,
                    left: self.borrow().body.x() - COHESION_RADIUS,
                };
            }
            let mut callback = |body: &Rc<BodyTrait>| {
                let body = &*body;
                if body.body_type() == BodyType::Boid {
                    let delta_angle = (body.angle() - self.angle()).rem(PI);
                    if delta_angle.abs() < COHESION_MAX_DELTA_ANGLE {
                        counter += 1;
                        sum += delta_angle;
                    }
                }
            };
            let this = self.borrow();
            this.world_batch.borrow().apply_locally(&location,&mut callback);
        }
        if counter > 0 {
            let a = self.angle() + dt*COHESION_FACTOR*sum/(counter as f64);
            self.set_angle(a);
        }
        self.borrow_mut().body.update(dt);
    }

    fn on_collision(&self, other: &BodyTrait) {
        other.damage(DAMAGE);
    }

    fn damage(&self, _: f64) {
        self.borrow_mut().alive = false;
    }
}
