use world::body::{Location, CollisionBehavior, PhysicType, Body};
use world::{World, Entity, EntityCell};
use std::cell::{RefCell, Ref, RefMut};
use frame_manager::{FrameManager, Animation};
use effect_manager::{EffectManager, Effect, Position};
use super::group;
use utils;

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
pub const DECISION_TIME: f64 = 0.2;

const DELTA_TIME_ANIMATION: f64 = 0.1;

enum State {
    Moving,
    Attacking,
    Attacking0,
    Attacking1,
    Attacking2,
    Attacking3,
}

pub struct Spider {
    body: Body,
    last_decision: f64,
    state: State,
    animation_counter: f64,
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
            last_decision: 0.,
            state: State::Moving,
            animation_counter: 0.,
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
        let take_decision = self.borrow().last_decision >= DECISION_TIME;
        if take_decision {
            self.borrow_mut().last_decision = 0.;
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
                if (x-prey_x).powi(2) + (y-prey_y).powi(2) < world.unit.powi(2) {
                    let angle = utils::Point {
                        x: prey_x-x,
                        y: prey_y-y,
                    }.angle_0x();
                    self.borrow_mut().body.angle = angle;
                    self.borrow_mut().body.velocity = VELOCITY;
                } else if let Some(angle) = utils::get_path_angle(x,y,prey_x,prey_y,VISIBLE_RADIUS,world.unit,&world.wall_map) {
                    self.borrow_mut().body.angle = angle;
                    self.borrow_mut().body.velocity = VELOCITY;
                } else {
                    self.borrow_mut().body.velocity = 0.;
                }
            } else {
                self.borrow_mut().body.velocity = 0.;
            }
        } else {
            self.borrow_mut().last_decision += dt;
        }

        let mut this = self.borrow_mut();
        this.state = match this.state {
            State::Moving => State::Moving,
            State::Attacking => {
                effect_manager.add(Effect::WaspAttack(Position::new(this.body.x,this.body.y)));
                this.animation_counter += dt;
                if this.animation_counter >= DELTA_TIME_ANIMATION {
                    this.animation_counter = 0.;
                    State::Attacking1
                } else {
                    State::Attacking0
                }
            }
            State::Attacking0 => {
                this.animation_counter += dt;
                if this.animation_counter >= DELTA_TIME_ANIMATION {
                    this.animation_counter = 0.;
                    State::Attacking1
                } else {
                    State::Attacking0
                }
            }
            State::Attacking1 => {
                this.animation_counter += dt;
                if this.animation_counter >= DELTA_TIME_ANIMATION {
                    this.animation_counter = 0.;
                    State::Attacking2
                } else {
                    State::Attacking1
                }
            }
            State::Attacking2 => {
                this.animation_counter += dt;
                if this.animation_counter >= DELTA_TIME_ANIMATION {
                    this.animation_counter = 0.;
                    State::Attacking3
                } else {
                    State::Attacking2
                }
            }
            State::Attacking3 => {
                this.animation_counter += dt;
                if this.animation_counter >= DELTA_TIME_ANIMATION {
                    this.animation_counter = 0.;
                    State::Moving
                } else {
                    State::Attacking3
                }
            }
        };

        if this.body.life <= 0. {
            effect_manager.add(Effect::WaspDeath(Position::new(this.body.x,this.body.y)));
        }
        this.body.update(dt);
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
        match self.state {
            State::Moving => frame_manager.draw_animation(self.body.x,self.body.y,self.body.angle,Animation::Wasp),
            State::Attacking => frame_manager.draw_animation(self.body.x,self.body.y,self.body.angle,Animation::WaspAttack0),
            State::Attacking0 => frame_manager.draw_animation(self.body.x,self.body.y,self.body.angle,Animation::WaspAttack0),
            State::Attacking1 => frame_manager.draw_animation(self.body.x,self.body.y,self.body.angle,Animation::WaspAttack1),
            State::Attacking2 => frame_manager.draw_animation(self.body.x,self.body.y,self.body.angle,Animation::WaspAttack2),
            State::Attacking3 => frame_manager.draw_animation(self.body.x,self.body.y,self.body.angle,Animation::WaspAttack3),
        }
        // self.body.render(color::RED,frame_manager);
    }
    fn on_collision(&mut self, other: &mut Entity) {
        if let State::Moving = self.state  {
            other.mut_body().damage(DAMAGE);
            self.state = State::Attacking;
        }
    }
}

