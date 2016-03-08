use super::{
    Body,
    BodyTrait,
    BodyType,
    CollisionBehavior,
};
use world::batch::Batch;
use util::direction::Direction;
use frame_manager::{
    color,
    FrameManager,
};
use effect_manager::{EffectManager, Position, Effect};
use rand::distributions::{IndependentSample, Range};
use rand;

use std::rc::Rc;
use std::collections::HashSet;
use std::cell::RefCell;

pub struct MovingWall {
    body: Body,
    unit: f64,
    direction: Direction,
    last_position: [f64;2],
    no_decision_time: f64,
    crushing: Option<(f64,f64,f64,f64)>,
}

pub const SIZE_RATIO: f64 = 0.999;
pub const WEIGHT: f64 = 1000.;
pub const VELOCITY: f64 = 35.;
pub const MASK: u32 = !0;
pub const GROUP: u32 = super::MOVING_WALL_GROUP;
pub const VIEW_RANGE: i32 = 4;
pub const TIMEOUT: f64 = 1.;

impl MovingWall {
    pub fn new(id: usize, x: i32, y: i32, angle: Direction, unit: f64) -> MovingWall {
        MovingWall {
            body: Body {
                id: id,
                x: (x as f64 + 0.5) * unit,
                y: (y as f64 + 0.5) * unit,
                width: unit * SIZE_RATIO,
                height: unit * SIZE_RATIO,
                weight: WEIGHT,
                velocity: VELOCITY,
                angle: angle.to_f64(),
                mask: MASK,
                group: GROUP,
                collision_behavior: CollisionBehavior::Persist,
                body_type: BodyType::MovingWall,
            },
            unit: unit,
            no_decision_time: 0.,
            direction: angle,
            last_position: [(x as f64 + 0.5)*unit,(y as f64 + 0.5)*unit],
            crushing: None,
        }
    }

    pub fn render(&mut self, frame_manager: &mut FrameManager) {
        self.body.render(color::RED,frame_manager);
    }
}

fn free_directions(id: usize, x: i32, y: i32, unit: f64, moving_walls: &Vec<Rc<RefCell<MovingWall>>>, wall_map: &HashSet<[i32;2]>) -> Vec<Direction> {
    let mut free_dir = Vec::new();

    let moving_walls_pos = {
        let mut vec = Vec::new();
        for mv in moving_walls {
            let mv_id = mv.borrow().id();
            let mv_x = mv.borrow().x()/unit;
            let mv_y = mv.borrow().y()/unit;
            let mv_dir = mv.borrow().direction;
            vec.push((mv_id,mv_x,mv_y,mv_dir));
        }
        vec
    };

    for dir in vec![Direction::Up,Direction::Right,Direction::Left,Direction::Down] {
        let index = match dir {
            Direction::Up => [x, y + 1],
            Direction::Down => [x, y - 1],
            Direction::Left => [x - 1, y],
            Direction::Right => [x + 1, y],
        };
        let moving_wall_blocking = moving_walls_pos.iter().any(|&(mv_id,mv_x,mv_y,mv_dir)| {
            if id != mv_id && (mv_x-index[0] as f64).abs() < 1. && (mv_y-index[1] as f64).abs() < 1. {
                true
            } else {
                false
            }
        });

        if !wall_map.contains(&index) && !moving_wall_blocking {
            free_dir.push(dir);
        }
    }

    free_dir
}

pub trait MovingWallManager {
    fn update(&self, dt: f64, batch: &Batch, moving_walls: &Vec<Rc<RefCell<MovingWall>>>, wall_map: &HashSet<[i32;2]>);
}

impl MovingWallManager for RefCell<MovingWall> {

    fn update(&self, dt: f64, batch: &Batch, moving_walls: &Vec<Rc<RefCell<MovingWall>>>, wall_map: &HashSet<[i32;2]>) {
        if let Some((x,y,angle,length)) = self.borrow().crushing {
            //TODO crushing
            self.borrow_mut().crushing = None;
        }

        let take_decision = if self.borrow().no_decision_time > TIMEOUT {
            true
        } else {
            let this = self.borrow();

            match this.direction {
                Direction::Up => {
                    this.last_position[1] + this.unit < this.body.y()
                },
                Direction::Down => {
                    this.last_position[1] - this.unit > this.body.y()
                },
                Direction::Left => {
                    this.last_position[0] - this.unit > this.body.x()
                },
                Direction::Right => {
                    this.last_position[0] + this.unit < this.body.x()
                },
            }
        };

        if take_decision {
            let next_dir = {
                let (id,x,y,unit) = {
                    let this = self.borrow();
                    let x_i32 = (this.x()/this.unit).floor() as i32;
                    let y_i32 = (this.y()/this.unit).floor() as i32;
                    (this.id(),x_i32,y_i32,this.unit)
                };
                let mut free_dir = free_directions(id,x,y,unit,moving_walls,wall_map);

                let this = self.borrow();

                let mut next_dir = if free_dir.len() > 0 {
                    let free_opposite = free_dir.contains(&this.direction.opposite());
                    free_dir.retain(|&dir| {
                        dir != this.direction.opposite()
                    });
                    if free_dir.len() > 0 {
                        let mut rng = rand::thread_rng();
                        let range = Range::new(0,free_dir.len());
                        let i = range.ind_sample(&mut rng);
                        Some(free_dir[i])
                    } else if free_opposite {
                        Some(this.direction.opposite())
                    } else {
                        None
                    }
                } else {
                    None
                };

                next_dir
            };
            let mut this = self.borrow_mut();
            let x = ((this.body.x()/this.unit).floor() + 0.5)*this.unit;
            let y = ((this.body.y()/this.unit).floor() + 0.5)*this.unit;
            this.last_position = [x,y];
            this.body.set_x(x);
            this.body.set_y(y);
            this.no_decision_time = 0.;
            if let Some(next_dir) = next_dir {
                this.direction = next_dir;
                this.body.angle = next_dir.to_f64();
                this.body.velocity = VELOCITY;
            } else {
                this.body.velocity = 0.;
            }

        } else {
            self.borrow_mut().no_decision_time += dt;
        }

        self.borrow_mut().body.update(dt);
    }
}

impl BodyTrait for MovingWall {
    delegate!{
        body:
            id() -> usize,
            dead() -> bool,
            body_type() -> BodyType,
            mut damage(d: f64) -> (),
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

    fn on_collision(&mut self, other: &mut BodyTrait) {
        //match other.body_type() {
        //    BodyType::Wall | BodyType::MovingWall => {
        //        //TODO crushing
        //        self.crushing = Some((1.,1.,1.,1.));
        //    },
        //    _ => (),
        //}
    }
}
