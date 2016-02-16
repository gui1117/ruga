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
use sound_manager::SoundManager;
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
}

pub const SIZE_RATIO: f64 = 0.99;
pub const WEIGHT: f64 = 1000.;
pub const VELOCITY: f64 = 35.;
pub const MASK: u32 = !0;
pub const GROUP: u32 = super::MOVING_WALL_GROUP;
pub const VIEW_RANGE: i32 = 4;

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

    pub fn render(&mut self, frame_manager: &mut FrameManager, sound_manager: &mut SoundManager) {
        self.body.render(color::RED,frame_manager);
    }
}

fn free_directions(x: i32, y: i32, direction: Direction, unit: f64, moving_walls: &Vec<Rc<RefCell<MovingWall>>>, wall_map: &HashSet<[i32;2]>) -> Vec<Direction> {
    let mut free_dir = Vec::new();

    let check_dir = match direction {
        Direction::Up => vec![Direction::Up,Direction::Right,Direction::Left],
        Direction::Down => vec![Direction::Right,Direction::Left,Direction::Down],
        Direction::Left => vec![Direction::Up,Direction::Left,Direction::Down],
        Direction::Right => vec![Direction::Up,Direction::Right,Direction::Down],
    };

    let moving_walls_pos = {
        let mut vec = Vec::new();
        for mv in moving_walls {
            let mv_x = mv.borrow().x()/unit;
            let mv_y = mv.borrow().y()/unit;
            let mv_dir = mv.borrow().direction;
            vec.push((mv_x,mv_y,mv_dir));
        }
        vec
    };

    for dir in check_dir {
        let index = match dir {
            Direction::Up => [x, y + 1],
            Direction::Down => [x, y - 1],
            Direction::Left => [x - 1, y],
            Direction::Right => [x + 1, y],
        };
        let moving_wall_blocking = moving_walls_pos.iter().any(|&(mv_x,mv_y,mv_dir)| {
            if (mv_x-x as f64).abs() < 0.5 && (mv_y-y as f64).abs() < 0.5 && dir != mv_dir {
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
    fn update(&self, dt: f64, moving_walls: &Vec<Rc<RefCell<MovingWall>>>, wall_map: &HashSet<[i32;2]>);
}

impl MovingWallManager for RefCell<MovingWall> {

    fn update(&self, dt: f64, moving_walls: &Vec<Rc<RefCell<MovingWall>>>, wall_map: &HashSet<[i32;2]>) {
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
                let (x,y,direction,unit) = {
                    let this = self.borrow();
                    let x_i32 = (this.x()/this.unit).floor() as i32;
                    let y_i32 = (this.y()/this.unit).floor() as i32;
                    (x_i32,y_i32,this.direction,this.unit)
                };
                let free_dir = free_directions(x,y,direction,unit,moving_walls,wall_map);

                let this = self.borrow();

                if !free_dir.contains(&this.direction) {
                    println!("ecrase ?");
                }

                let mut next_dir = if free_dir.len() > 0 {
                    let mut rng = rand::thread_rng();
                    let range = Range::new(0,free_dir.len());
                    let i = range.ind_sample(&mut rng);
                    free_dir[i]
                } else {
                    this.direction.opposite()
                };

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
            mut on_collision(other: &mut BodyTrait) -> (),
    }
}
