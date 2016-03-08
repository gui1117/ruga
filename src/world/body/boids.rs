use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashSet;
use std::f64::consts::PI;
use util::minus_pi_pi;
use frame_manager::{
    color,
    FrameManager,
};
use rand::distributions::{IndependentSample, Range};
use rand;

use super::{
    Body,
    BodyTrait,
    CollisionBehavior,
    BodyType,
};

pub const NUMBER_OF_BOID: usize = 30;

pub const LIFE: f64 = 1.;
pub const WIDTH: f64 = 1.;
pub const HEIGHT: f64 = 1.;
pub const WEIGHT: f64 = 1.;
pub const MASK: u32 = !0;
pub const GROUP: u32 = super::BOID_GROUP;
pub const DAMAGE: f64 = 0.4;
pub const VELOCITY: f64 = 20.;

pub const COHESION_RADIUS: f64 = 10.;
pub const COHESION_MAX_DELTA_ANGLE: f64 = PI;
pub const COHESION_FACTOR: f64 = 5.;

pub const LIVING_RADIUS: f64 = 60.;
pub const BORNING_RADIUS: f64 = 40.;

pub fn boid_generator(n: usize, character_pos: [f64;2], wall_map: &HashSet<[i32;2]>, unit: f64) -> Vec<(f64,f64,f64)> {
    if n < NUMBER_OF_BOID {
        let free = {
            let bound_up = ((character_pos[0]-BORNING_RADIUS)/unit).floor() as i32;
            let bound_down = ((character_pos[0]+BORNING_RADIUS)/unit).floor() as i32;
            let bound_left = ((character_pos[1]-BORNING_RADIUS)/unit).floor() as i32;
            let bound_right = ((character_pos[1]+BORNING_RADIUS)/unit).floor() as i32;
            let mut vec = Vec::new();
            for i in bound_up..bound_down+1 {
                // if not ! then boid comes from wall
                // it may be cool...
                if !wall_map.contains(&[i,bound_left]) {
                    vec.push([i,bound_left]);
                }
                if !wall_map.contains(&[i,bound_right]) {
                    vec.push([i,bound_right]);
                }
            }
            for j in bound_left+1..bound_right {
                if !wall_map.contains(&[bound_up,j]) {
                    vec.push([bound_up,j]);
                }
                if !wall_map.contains(&[bound_down,j]) {
                    vec.push([bound_down,j]);
                }
            }

            vec
        };
        if free.len() == 0 { return Vec::new(); }

        let mut to_create = Vec::new();

        let mut rng = rand::thread_rng();
        let free_range = Range::new(0,free.len());
        let unit_range = Range::new(0.,unit);
        let angle_range = Range::new(0.,6.28);

        for _ in 0..NUMBER_OF_BOID-n {
            let index = free_range.ind_sample(&mut rng);
            let x = unit_range.ind_sample(&mut rng) + free[index][0] as f64*unit;
            let y = unit_range.ind_sample(&mut rng) + free[index][1] as f64*unit;
            let angle = angle_range.ind_sample(&mut rng);

            to_create.push((x,y,angle));
        }

        to_create
    } else {
        Vec::new()
    }
}

pub struct Boid {
    body: Body,
    life: f64,
}

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

    pub fn render(&mut self, frame_manager: &mut FrameManager) {
        self.body.render(color::RED,frame_manager);
    }
}

pub trait BoidManager {
    fn update(&self, dt: f64, character_pos: [f64;2], boids: &Vec<Rc<RefCell<Boid>>>);
}

impl BoidManager for RefCell<Boid> {
    fn update(&self, dt: f64, character_pos: [f64;2], boids: &Vec<Rc<RefCell<Boid>>>) {
        {
            let mut this = self.borrow_mut();
            if (character_pos[0]-this.x()).abs() > LIVING_RADIUS || (character_pos[1]-this.y()).abs() > LIVING_RADIUS {
                this.life = 0.;
                return;
            }
        }

        let mut counter = 0;
        let mut sum = 0.;

        {
            let (id,x,y,angle) = {
                let this = self.borrow();
                (this.id(), this.body.x(), this.body.y(), this.angle())
            };

            for boid in boids {
                let boid = boid.borrow();
                if boid.id() != id && (boid.x()-x).abs() < COHESION_RADIUS && (boid.y()-y).abs() < COHESION_RADIUS {
                    let delta_angle = minus_pi_pi(boid.angle() - angle);
                    if delta_angle.abs() < COHESION_MAX_DELTA_ANGLE {
                        counter += 1;
                        sum += delta_angle;
                    }
                }
            }
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
