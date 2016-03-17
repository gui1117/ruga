// NOT COMPLETE

use utils::Direction;
use frame_manager::{FrameManager, Animation};
use effect_manager::{EffectManager, Position, Effect};
use rand::distributions::{IndependentSample, Range};
use world::{World, Entity, EntityCell};
use world::body::{CollisionBehavior, PhysicType, Body};
use rand;

use std::cell::{RefCell, Ref, RefMut};
use std::f64;

pub struct BurningWall {
    body: Body,
    unit: f64,
    direction: Direction,
    last_position: [f64;2],
    no_decision_time: f64,
}

pub const SIZE_RATIO: f64 = 1.;
pub const WEIGHT: f64 = 1000.;
pub const VELOCITY: f64 = 35.;
pub const MASK: u64 = !super::group::WALL_KIND;
pub const GROUP: u64 = super::group::BURNING_WALL;
pub const COLLISION_BEHAVIOR: CollisionBehavior = CollisionBehavior::Persist;
pub const PHYSIC_TYPE: PhysicType = PhysicType::Dynamic;
pub const DAMAGE: f64 = 1000.;

impl BurningWall {
    pub fn new(x: i32, y: i32, unit: f64) -> BurningWall {
        BurningWall {
            body: Body {
                id: 0,
                items: Vec::new(),
                x: (x as f64 + 0.5) * unit,
                y: (y as f64 + 0.5) * unit,
                life: f64::MAX,
                width: unit * SIZE_RATIO,
                height: unit * SIZE_RATIO,
                weight: WEIGHT,
                velocity: 0.,
                angle: 0.,
                mask: MASK,
                group: GROUP,
                collision_behavior: COLLISION_BEHAVIOR,
                physic_type: PHYSIC_TYPE,
            },
            unit: unit,
            no_decision_time: 0.,
            direction: Direction::Right,
            last_position: [(x as f64 + 0.5)*unit,(y as f64 + 0.5)*unit],
        }
    }
}

impl BurningWall {
    fn coordinate(&self) -> (i32,i32) {
        ((self.body.x/self.unit).floor() as i32, (self.body.y/self.unit).floor() as i32)
    }
}

impl EntityCell for RefCell<BurningWall> {
    fn borrow(&self) -> Ref<Entity> {
        (self as &RefCell<Entity>).borrow()
    }
    fn borrow_mut(&self) -> RefMut<Entity> {
        (self as &RefCell<Entity>).borrow_mut()
    }
    fn update(&self, dt: f64, world: &World, effect_manager: &mut EffectManager) {
        let velocity = self.borrow_mut().body.velocity;
        if velocity == 0. {
            let (x,y) = self.borrow().coordinate();
            let free_directions = free_directions(x,y,world);
            if free_directions.len() == 0 {
                self.borrow_mut().body.life = 0.;
            } else {
                let mut rng = rand::thread_rng();
                let range = Range::new(0,free_directions.len());
                let i = range.ind_sample(&mut rng);
                self.borrow_mut().body.velocity = VELOCITY;
                self.borrow_mut().body.angle = free_directions[i].to_f64();
                self.borrow_mut().direction = free_directions[i];
            }
        } else {
            let take_decision = {
                let this = self.borrow();
                match this.direction {
                    Direction::Up => {
                        this.last_position[1] + this.unit < this.body.y
                    },
                    Direction::Down => {
                        this.last_position[1] - this.unit > this.body.y
                    },
                    Direction::Left => {
                        this.last_position[0] - this.unit > this.body.x
                    },
                    Direction::Right => {
                        this.last_position[0] + this.unit < this.body.x
                    },
                }
            };
            if take_decision {
                let (x,y) = self.borrow().coordinate();
                let mut free_directions = free_directions(x,y,world);

                let mut this = self.borrow_mut();
                if free_directions.len() >= 2 {
                    free_directions.retain(|&direction| direction != this.direction.opposite());
                }

                if free_directions.len() == 0 {
                    this.body.life = 0.;
                } else {
                    let mut rng = rand::thread_rng();
                    let range = Range::new(0,free_directions.len());
                    let i = range.ind_sample(&mut rng);
                    this.direction = free_directions[i];
                    this.body.angle = this.direction.to_f64();
                }
                let x = ((this.body.x/this.unit).floor() + 0.5)*this.unit;
                let y = ((this.body.y/this.unit).floor() + 0.5)*this.unit;
                this.last_position = [x,y];
                this.no_decision_time = 0.;
                effect_manager.add(Effect::BurningWallDecision(Position::new(this.body.x,this.body.y)));
            }
        }

        // update position
        self.borrow_mut().body.update(dt);

        // reset grid aligned
        {
            let mut this = self.borrow_mut();
            if this.direction.perpendicular(&Direction::Up) {
                this.body.y = ((this.body.y/this.unit-0.5).round()+0.5)*this.unit;
            } else {
                this.body.x = ((this.body.x/this.unit-0.5).round()+0.5)*this.unit;
            }
        }

    }
}

impl Entity for BurningWall {
    fn body(&self) -> &Body {
        &self.body
    }
    fn mut_body(&mut self) -> &mut Body {
        &mut self.body
    }
    fn render(&self, frame_manager: &mut FrameManager) {
         frame_manager.draw_animation(self.body.x,self.body.y,self.body.angle,Animation::BurningWall);
        //self.body.render(color::RED,frame_manager);
    }
    fn on_collision(&mut self, other: &mut Entity) {
        other.mut_body().damage(DAMAGE);
    }
}

fn free_directions(x: i32, y: i32, world: &World) -> Vec<Direction> {
    let mut directions = Vec::new();
    for &direction in &[Direction::Up,Direction::Down,Direction::Left,Direction::Right] {
        let index = match direction {
            Direction::Up => [x, y + 1],
            Direction::Down => [x, y - 1],
            Direction::Left => [x - 1, y],
            Direction::Right => [x + 1, y],
        };
        if !world.wall_map.contains(&(index[0],index[1])) {
            directions.push(direction);
        }
    }
    directions
}

