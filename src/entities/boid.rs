//! boid are generated all the time around the character
//! they die if they are too far from a character
//! it forces the player to always move

use world::body::{Location, CollisionBehavior, PhysicType, Body};
use world::{World, Entity, EntityCell};
use std::cell::{RefCell, Ref, RefMut};
use std::f64::consts::PI;
use utils::minus_pi_pi;
use super::group;
use frame_manager::{FrameManager, Animation};
use effect_manager::{EffectManager, Position, Effect};
// use rand::distributions::{IndependentSample, Range};
// use rand;

// pub const NUMBER_OF_BOID: usize = 30;

pub const LIFE: f64 = 1.;
pub const WIDTH: f64 = 1.;
pub const HEIGHT: f64 = 1.;
pub const WEIGHT: f64 = 1.;
pub const MASK: u64 = !group::BOID;
pub const GROUP: u64 = group::BOID;
pub const DAMAGE: f64 = 0.4;
pub const VELOCITY: f64 = 20.;
pub const COLLISION_BEHAVIOR: CollisionBehavior = CollisionBehavior::Bounce;
pub const PHYSIC_TYPE: PhysicType = PhysicType::Dynamic;

pub const COHESION_RADIUS: f64 = 10.;
pub const COHESION_MAX_DELTA_ANGLE: f64 = PI;
pub const COHESION_FACTOR: f64 = 5.;

pub const LIVING_RADIUS: f64 = 60.;

pub struct Boid {
    body: Body,
}

impl Boid {
    pub fn new(x: f64, y: f64, angle: f64) -> Boid {
        Boid {
            body: Body {
                id: 0,
                x: x,
                y: y,
                life: LIFE,
                items: Vec::new(),
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
        }
    }
}

impl EntityCell for RefCell<Boid> {
    fn borrow(&self) -> Ref<Entity> {
        (self as &RefCell<Entity>).borrow()
    }
    fn borrow_mut(&self) -> RefMut<Entity> {
        (self as &RefCell<Entity>).borrow_mut()
    }
    fn update(&self, dt: f64, world: &World, effect_manager: &mut EffectManager) {
        let mut counter = 0;
        let mut sum = 0.;
        let mut character = false;

        let (id,x,y,angle) = {
            let this = self.borrow();
            let body = this.body();
            (body.id, body.x, body.y, body.angle)
        };
        let loc = Location {
            up: y + LIVING_RADIUS,
            down: y - LIVING_RADIUS,
            left: x - LIVING_RADIUS,
            right: x + LIVING_RADIUS,
        };

        world.apply_locally(group::BOID | group::CHARACTER, &loc, &mut |entity: &mut Entity| {
            let body = entity.body();
            match body.group {
                group::BOID => {
                    if body.id != id && (body.x-x).abs() < COHESION_RADIUS && (body.y-y).abs() < COHESION_RADIUS {
                        let delta_angle = minus_pi_pi(body.angle - angle);
                        if delta_angle.abs() < COHESION_MAX_DELTA_ANGLE {
                            counter += 1;
                            sum += delta_angle;
                        }
                    }
                },
                group::CHARACTER => {
                    character = true;
                },
                _ => panic!("impossible arm"),
            }
        });

        let mut this = self.borrow_mut();
        let body = this.mut_body();
        if counter > 0 {
            body.angle -= dt*COHESION_FACTOR*sum/(counter as f64);
        }
        body.update(dt);

        if body.life <= 0. {
            effect_manager.add(Effect::BoidExplosion(Position::new(body.x,body.y)));
        }
    }
}

impl Entity for Boid {
    fn body(&self) -> &Body {
        &self.body
    }
    fn mut_body(&mut self) -> &mut Body {
        &mut self.body
    }
    fn render(&self, frame_manager: &mut FrameManager) {
        frame_manager.draw_animation(self.body.x,self.body.y,self.body.angle,Animation::Boid);
        // self.body.render(color::RED, frame_manager);
    }
    fn on_collision(&mut self, other: &mut Entity) {
        other.mut_body().damage(DAMAGE);
    }
}

// pub fn boid_generator(n: usize, character_pos: [f64;2], wall_map: &HashSet<[i32;2]>, unit: f64) -> Vec<(f64,f64,f64)> {
//     if n < NUMBER_OF_BOID {
//         let free = {
//             let bound_up = ((character_pos[0]-BORNING_RADIUS)/unit).floor() as i32;
//             let bound_down = ((character_pos[0]+BORNING_RADIUS)/unit).floor() as i32;
//             let bound_left = ((character_pos[1]-BORNING_RADIUS)/unit).floor() as i32;
//             let bound_right = ((character_pos[1]+BORNING_RADIUS)/unit).floor() as i32;
//             let mut vec = Vec::new();
//             for i in bound_up..bound_down+1 {
//                 // if not ! then boid comes from wall
//                 // it may be cool...
//                 if !wall_map.contains(&[i,bound_left]) {
//                     vec.push([i,bound_left]);
//                 }
//                 if !wall_map.contains(&[i,bound_right]) {
//                     vec.push([i,bound_right]);
//                 }
//             }
//             for j in bound_left+1..bound_right {
//                 if !wall_map.contains(&[bound_up,j]) {
//                     vec.push([bound_up,j]);
//                 }
//                 if !wall_map.contains(&[bound_down,j]) {
//                     vec.push([bound_down,j]);
//                 }
//             }

//             vec
//         };
//         if free.len() == 0 { return Vec::new(); }

//         let mut to_create = Vec::new();

//         let mut rng = rand::thread_rng();
//         let free_range = Range::new(0,free.len());
//         let unit_range = Range::new(0.,unit);
//         let angle_range = Range::new(0.,6.28);

//         for _ in 0..NUMBER_OF_BOID-n {
//             let index = free_range.ind_sample(&mut rng);
//             let x = unit_range.ind_sample(&mut rng) + free[index][0] as f64*unit;
//             let y = unit_range.ind_sample(&mut rng) + free[index][1] as f64*unit;
//             let angle = angle_range.ind_sample(&mut rng);

//             to_create.push((x,y,angle));
//         }

//         to_create
//     } else {
//         Vec::new()
//     }
// }

