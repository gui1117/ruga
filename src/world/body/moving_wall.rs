use super::{
    Body,
    BodyTrait,
    BodyType,
    CollisionBehavior,
};
use world::batch::Batch;
use util::direction::Direction;

use std::rc::Rc;
use std::cell::RefCell;

pub struct MovingWall {
    body: Body,
    unit: f64,
    direction: Direction,
    last_position: [f64;2],
}

pub const SIZE_RATIO: f64 = 0.8;
pub const WEIGHT: f64 = 10000.;
pub const VELOCITY: f64 = 200.;
pub const MASK: u32 = !0;
pub const GROUP: u32 = super::MOVING_WALL_GROUP;
pub const VIEW_RANGE: i32 = 4;
pub const DAMAGE: f64 = 100.;

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
            direction: angle,
            last_position: [(x as f64 + 0.5)*unit,(y as f64 + 0.5)*unit],
        }
    }

    fn free_directions(&self, batch: &Batch) -> Vec<Direction> {
        let mut free_dir = Vec::new();

        let check_free = |body_vec: Vec<Rc<RefCell<BodyTrait>>>| {
            for body in body_vec {
                match body.borrow().body_type() {
                    BodyType::MovingWall | BodyType::Wall => {
                        return false;
                    },
                    _ => (),
                }
            }
            true
        };

        let check_dir = match self.direction {
            Direction::Up => vec![Direction::Up,Direction::Right,Direction::Left],
            Direction::Down => vec![Direction::Right,Direction::Left,Direction::Down],
            Direction::Left => vec![Direction::Up,Direction::Left,Direction::Down],
            Direction::Right => vec![Direction::Up,Direction::Right,Direction::Down],
        };

        let x_i32 = (self.body.x()/self.unit).floor() as i32;
        let y_i32 = (self.body.y()/self.unit).floor() as i32;
        for dir in check_dir {
            let index = match dir {
                Direction::Up => [x_i32, y_i32 + 1],
                Direction::Down => [x_i32, y_i32 - 1],
                Direction::Left => [x_i32 - 1, y_i32],
                Direction::Right => [x_i32 + 1, y_i32],
            };
            if check_free(batch.get_on_index(&index)) {
                free_dir.push(dir);
            }
        }

        free_dir
    }

    fn visible_prey(&self, dir: Direction, batch: &Batch) -> Option<i32>  {
        let x_i32 = (self.body.x()/self.unit).floor() as i32;
        let y_i32 = (self.body.y()/self.unit).floor() as i32;

        let index_vec = match dir {
            Direction::Up => {
                let mut vec = Vec::new();
                let mut y = y_i32;
                for _ in 0..VIEW_RANGE {
                    y += 1;
                    vec.push([x_i32,y]);
                }
                vec
            },
            Direction::Down => {
                let mut vec = Vec::new();
                let mut y = y_i32;
                for _ in 0..VIEW_RANGE {
                    y -= 1;
                    vec.push([x_i32,y]);
                }
                vec
            },
            Direction::Left => {
                let mut vec = Vec::new();
                let mut x = x_i32;
                for _ in 0..VIEW_RANGE {
                    x -= 1;
                    vec.push([x,y_i32]);
                }
                vec
            },
            Direction::Right => {
                let mut vec = Vec::new();
                let mut x = x_i32;
                for _ in 0..VIEW_RANGE {
                    x += 1;
                    vec.push([x,y_i32]);
                }
                vec
            },
        };

        let mut distance = 0;
        for index in &index_vec {
            distance += 1;
            let body_vec = batch.get_on_index(index);
            for body in body_vec {
                if body.borrow().body_type() == BodyType::Wall {
                    return None;
                } else if body.borrow().body_type() == BodyType::Character {
                    return Some(distance);
                }
            }
        }
        None
    }

    pub fn render_debug(&self, lines: &mut Vec<[f64;4]>) {
        self.body.render_debug(lines);
    }
}

pub trait MovingWallManager {
    fn update(&self, dt: f64, batch: &Batch);
}

impl MovingWallManager for RefCell<MovingWall> {

    fn update(&self, dt: f64, batch: &Batch) {
        use std::i32;

        let take_decision = {
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
                let this = self.borrow();
                let free_dir = this.free_directions(batch);
                let mut next_dir = if free_dir.contains(&this.direction) {
                    this.direction
                } else if free_dir.len() > 0 {
                    free_dir[0]
                } else {
                    this.direction.opposite()
                };
                let mut closest_prey_dist = i32::MAX;
                for dir in free_dir {
                    if let Some(distance) = this.visible_prey(dir,batch) {
                        if distance < closest_prey_dist {
                            closest_prey_dist = distance;
                            next_dir = dir;
                        }
                    }
                }

                next_dir
            };
            let mut this = self.borrow_mut();
            let x = ((this.body.x()/this.unit).floor() + 0.5)*this.unit;
            let y = ((this.body.y()/this.unit).floor() + 0.5)*this.unit;
            this.last_position = [x,y];
            this.body.set_x(x);
            this.body.set_y(y);
            this.direction = next_dir;
            this.body.angle = next_dir.to_f64();
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
        other.damage(DAMAGE);
    }
}
