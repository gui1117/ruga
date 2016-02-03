use viewport::Viewport;
use opengl_graphics::GlGraphics;
use world::{ 
    Camera, 
};
use world::batch::Batch;
use world::spatial_hashing::Location;
use std::rc::Rc;
use std::cell::RefCell;
use super::{ 
    Body, 
    BodyTrait, 
    CollisionBehavior,
    BodyType,
};

pub const VELOCITY: f64 = 10.;
pub const TIME_TO_STOP: f64 = 0.8;
pub const TIME_TO_EXPLODE: f64 = TIME_TO_STOP + 1.;
pub const WIDTH: f64 = 1.;
pub const HEIGHT: f64 = 1.;
pub const DAMAGE: f64 = 10.;
pub const RADIUS: f64 = 10.;
pub const WEIGHT: f64 = 1.;
pub const GROUP: u32 = 64;
pub const MASK: u32 = !0;


pub struct Grenade {
    body: Body,
    timer: f64,
    alive: bool,
    batch: Rc<RefCell<Batch>>,
}

impl Grenade {
    pub fn new(id: usize, x: f64, y: f64, angle: f64, batch: Rc<RefCell<Batch>>) -> Grenade {
        Grenade {
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
                collision_behavior: CollisionBehavior::Random,
                body_type: BodyType::Grenade,
            },
            alive: true,
            timer: 0.,
            batch: batch,
        }
    }
}

impl BodyTrait for RefCell<Grenade> {
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

    fn dead(&self) -> bool {
        !self.borrow().alive
    }

    fn update(&self, dt: f64) {
        self.borrow_mut().timer += dt;
        let alive = self.borrow().alive;
        let timer = self.borrow().timer;
        if alive {
            if timer >= TIME_TO_EXPLODE {
                let batch = self.borrow().batch.clone();
                let loc = Location {
                    up: self.y() + RADIUS,
                    down: self.y() - RADIUS,
                    left: self.x() - RADIUS,
                    right: self.x() + RADIUS,
                };
                batch.borrow().apply_locally(&loc, &mut |body: &Rc<BodyTrait>| {
                    body.damage(DAMAGE);
                });
                self.borrow_mut().alive = false;
            } else if timer >= TIME_TO_STOP {
                self.set_velocity(0.);
            }
        }
        self.borrow_mut().body.update(dt);
    }

    fn on_collision(&self, _: &BodyTrait) {
        self.borrow_mut().timer = TIME_TO_EXPLODE;
    }

    fn damage(&self, _: f64) {
        self.borrow_mut().timer = TIME_TO_EXPLODE;
    }
}

