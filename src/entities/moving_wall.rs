// NOT COMPLETE

use utils::Direction;
use super::group;
use frame_manager::{color, FrameManager};
use effect_manager::EffectManager;
use rand::distributions::{IndependentSample, Range};
use world::{World, Entity, EntityCell};
use world::body::{CollisionBehavior, PhysicType, Body};
use rand;

use std::cell::{RefCell, Ref, RefMut};
use std::f64;

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
pub const MASK: u64 = !0;
pub const GROUP: u64 = super::group::MOVING_WALL;
pub const TIMEOUT: f64 = 1.; //TODO compute the time between two standards decisions
pub const COLLISION_BEHAVIOR: CollisionBehavior = CollisionBehavior::Persist;
pub const PHYSIC_TYPE: PhysicType = PhysicType::Dynamic;

impl MovingWall {
    pub fn new(x: i32, y: i32, unit: f64) -> MovingWall {
        MovingWall {
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
            crushing: None,
        }
    }
}

impl EntityCell for RefCell<MovingWall> {
    fn borrow(&self) -> Ref<Entity> {
        (self as &RefCell<Entity>).borrow()
    }
    fn borrow_mut(&self) -> RefMut<Entity> {
        (self as &RefCell<Entity>).borrow_mut()
    }
    fn update(&self, dt: f64, world: &World, _effect_manager: &mut EffectManager) {
        // reset grid aligned
        {
            let mut this = self.borrow_mut();
            if this.direction.perpendicular(&Direction::Up) {
                this.body.y = ((this.body.y/this.unit-0.5).round()+0.5)*this.unit;
            } else {
                this.body.x = ((this.body.x/this.unit-0.5).round()+0.5)*this.unit;
            }
        }

        // take decicion
        {
            let take_decision = if self.borrow().no_decision_time > TIMEOUT {
                true
            } else {
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
                let (x,y) = {
                    let this = self.borrow();
                    ((this.body.x/this.unit-0.5) as i32, (this.body.y/this.unit-0.5) as i32)
                };
                let mut free_directions = free_directions(x,y,world);

                let mut this = self.borrow_mut();
                if free_directions.len() >= 2 {
                    free_directions.retain(|&direction| direction != this.direction.opposite());
                }

                if free_directions.len() == 0 {
                    this.body.velocity = 0.;
                } else {
                    this.body.velocity = VELOCITY;
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
            } else {
                self.borrow_mut().no_decision_time += dt;
            }
        }

        // update position
        self.borrow_mut().body.update(dt);
    }
}

impl Entity for MovingWall {
    fn body(&self) -> &Body {
        &self.body
    }
    fn mut_body(&mut self) -> &mut Body {
        &mut self.body
    }
    fn render(&self, frame_manager: &mut FrameManager) {
        self.body.render(color::RED,frame_manager);
    }
    fn on_collision(&mut self, other: &mut Entity) {
        let other_body = other.body();
        if other_body.group & group::WALL_KIND != 0 {
            self.direction = self.direction.opposite();
            self.body.angle = -self.body.angle
                // TODO       self.crushing = Some((1.,1.,1.,1.));
        }
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
        let mut some_wall = false;
        world.apply_on_index(group::WALL, &index, &mut |_: &mut Entity| {
            some_wall = true;
        });
        if !some_wall {
            directions.push(direction);
        }
    }
    directions
}

// fn free_directions(id: usize, x: i32, y: i32, unit: f64, moving_walls: &Vec<Rc<RefCell<MovingWall>>>, wall_map: &HashSet<[i32;2]>) -> Vec<Direction> {
//     let mut free_dir = Vec::new();

//     let moving_walls_pos = {
//         let mut vec = Vec::new();
//         for mv in moving_walls {
//             let mv_id = mv.borrow().id();
//             let mv_x = mv.borrow().x()/unit;
//             let mv_y = mv.borrow().y()/unit;
//             let mv_dir = mv.borrow().direction;
//             vec.push((mv_id,mv_x,mv_y,mv_dir));
//         }
//         vec
//     };

//     for dir in vec![Direction::Up,Direction::Right,Direction::Left,Direction::Down] {
//         let index = match dir {
//             Direction::Up => [x, y + 1],
//             Direction::Down => [x, y - 1],
//             Direction::Left => [x - 1, y],
//             Direction::Right => [x + 1, y],
//         };
//         let moving_wall_blocking = moving_walls_pos.iter().any(|&(mv_id,mv_x,mv_y,mv_dir)| {
//             if id != mv_id && (mv_x-index[0] as f64).abs() < 1. && (mv_y-index[1] as f64).abs() < 1. {
//                 true
//             } else {
//                 false
//             }
//         });

//         if !wall_map.contains(&index) && !moving_wall_blocking {
//             free_dir.push(dir);
//         }
//     }

//     free_dir
// }

// pub trait MovingWallManager {
//     fn update(&self, dt: f64, batch: &Batch, moving_walls: &Vec<Rc<RefCell<MovingWall>>>, wall_map: &HashSet<[i32;2]>);
// }

// impl MovingWallManager for RefCell<MovingWall> {

//     fn update(&self, dt: f64, batch: &Batch, moving_walls: &Vec<Rc<RefCell<MovingWall>>>, wall_map: &HashSet<[i32;2]>) {
//         if let Some((x,y,angle,length)) = self.borrow().crushing {
//             //TODO crushing
//             self.borrow_mut().crushing = None;
//         }

//         let take_decision = if self.borrow().no_decision_time > TIMEOUT {
//             true
//         } else {
//             let this = self.borrow();

//             match this.direction {
//                 Direction::Up => {
//                     this.last_position[1] + this.unit < this.body.y()
//                 },
//                 Direction::Down => {
//                     this.last_position[1] - this.unit > this.body.y()
//                 },
//                 Direction::Left => {
//                     this.last_position[0] - this.unit > this.body.x()
//                 },
//                 Direction::Right => {
//                     this.last_position[0] + this.unit < this.body.x()
//                 },
//             }
//         };

//         if take_decision {
//             let next_dir = {
//                 let (id,x,y,unit) = {
//                     let this = self.borrow();
//                     let x_i32 = (this.x()/this.unit).floor() as i32;
//                     let y_i32 = (this.y()/this.unit).floor() as i32;
//                     (this.id(),x_i32,y_i32,this.unit)
//                 };
//                 let mut free_dir = free_directions(id,x,y,unit,moving_walls,wall_map);

//                 let this = self.borrow();

//                 let mut next_dir = if free_dir.len() > 0 {
//                     let free_opposite = free_dir.contains(&this.direction.opposite());
//                     free_dir.retain(|&dir| {
//                         dir != this.direction.opposite()
//                     });
//                     if free_dir.len() > 0 {
//                         let mut rng = rand::thread_rng();
//                         let range = Range::new(0,free_dir.len());
//                         let i = range.ind_sample(&mut rng);
//                         Some(free_dir[i])
//                     } else if free_opposite {
//                         Some(this.direction.opposite())
//                     } else {
//                         None
//                     }
//                 } else {
//                     None
//                 };

//                 next_dir
//             };
//             let mut this = self.borrow_mut();
//             let x = ((this.body.x()/this.unit).floor() + 0.5)*this.unit;
//             let y = ((this.body.y()/this.unit).floor() + 0.5)*this.unit;
//             this.last_position = [x,y];
//             this.body.set_x(x);
//             this.body.set_y(y);
//             this.no_decision_time = 0.;
//             if let Some(next_dir) = next_dir {
//                 this.direction = next_dir;
//                 this.body.angle = next_dir.to_f64();
//                 this.body.velocity = VELOCITY;
//             } else {
//                 this.body.velocity = 0.;
//             }

//         } else {
//             self.borrow_mut().no_decision_time += dt;
//         }

//         self.borrow_mut().body.update(dt);
//     }
// }
