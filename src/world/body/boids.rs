use viewport::Viewport;
use opengl_graphics::GlGraphics;
use world::{ 
    Camera, 
};

use super::{ 
    Body, 
    BodyTrait, 
    CollisionBehavior,
};

pub struct Boid {
    body: Body,
    alive: bool
}

pub const WIDTH: f64 = 10.;
pub const HEIGHT: f64 = 10.;
pub const WEIGHT: f64 = 1.;
pub const MASK: u32 = !0;
pub const GROUP: u32 = 4;
pub const DAMAGE: f64 = 10.;


impl Boid {
    pub fn new(id: usize, x: f64, y: f64, angle: f64) -> Boid {
        Boid {
            body: Body {
                id: id,
                x: x,
                y: y,
                width2: WIDTH/2.,
                height2: HEIGHT/2.,
                weight: WEIGHT,
                velocity: 0.,
                angle: angle,
                mask: MASK,
                group: GROUP,
                collision_behavior: CollisionBehavior::Bounce,
            },
            alive: true,
        }
    }
}

impl BodyTrait for Boid {
    delegate!{
        body:
           id() -> usize,
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
           mut update(dt: f64) -> (),
           collision_behavior() -> CollisionBehavior,
           render(viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) -> (),
    }

    fn on_collision(&mut self, other: &mut BodyTrait) {
        other.damage(DAMAGE);
    }

    fn damage(&mut self, _: f64) {
        self.alive = false;
    }
}
