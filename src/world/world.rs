use super::body::{ 
    Wall, 
    MovingWall, 
    Armory,
    Character, 
    Grenade,
    //Snake,
    Boid,
    BodyType,
    BodyTrait, 
};
use super::body::character::CharacterManager;
use super::body::moving_wall::MovingWallManager;
use super::body::grenade::GrenadeManager;
use super::body::boids::BoidManager;
use super::body::boids::boid_generator;
use super::batch::Batch;
use util::direction::Direction;
use sound_manager::SoundManager;
use frame_manager::FrameManager;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

pub struct World {
    pub time: f64,
    pub unit: f64,
    next_id: usize,
    /// whether there is a wall or not
    pub wall_map: HashSet<[i32;2]>,
    pub walls: Vec<Rc<RefCell<Wall>>>,
    pub armories: Vec<Rc<RefCell<Armory>>>,
    pub boids: Vec<Rc<RefCell<Boid>>>,
    pub grenades: Vec<Rc<RefCell<Grenade>>>,
    pub moving_walls: Vec<Rc<RefCell<MovingWall>>>,
    //pub snakes: Vec<Rc<RefCell<Snake>>>,
    pub characters: Vec<Rc<RefCell<Character>>>,
    pub static_vec: Vec<Rc<RefCell<BodyTrait>>>,
    pub dynamic_vec: Vec<Rc<RefCell<BodyTrait>>>,
    pub batch: Batch,
}

impl World {
    pub fn new(unit: f64) -> World {
        World {
            unit: unit,
            time: 0.,
            next_id: 1,
            characters: Vec::new(),
            moving_walls: Vec::new(),
            armories: Vec::new(),
            //snakes: Vec::new(),
            boids: Vec::new(),
            grenades: Vec::new(),
            walls: Vec::new(),
            static_vec: Vec::new(),
            dynamic_vec: Vec::new(),
            batch: Batch::new(unit),
            wall_map: HashSet::new(),
        }
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn insert_armory(&mut self, x: i32, y: i32) {
        let armory = Rc::new(RefCell::new(Armory::new(self.next_id(),x,y,self.unit)));
        let a_armory = armory.clone() as Rc<RefCell<BodyTrait>>;
        self.batch.insert_static(&a_armory);
        self.static_vec.push(a_armory);
        self.armories.push(armory);
    }

    pub fn insert_wall(&mut self, x: i32, y: i32) {
        self.wall_map.insert([x,y]);

        let wall = Rc::new(RefCell::new(Wall::new(self.next_id(),x,y,self.unit)));
        let a_wall = wall.clone() as Rc<RefCell<BodyTrait>>;
        self.batch.insert_static(&a_wall);
        self.static_vec.push(a_wall);
        self.walls.push(wall);
    }

    pub fn insert_grenade(&mut self, x: f64, y: f64, angle: f64) {
        let grenade = Rc::new(RefCell::new(Grenade::new(self.next_id(),x,y,angle)));
        let a_grenade = grenade.clone() as Rc<RefCell<BodyTrait>>;
        self.batch.insert_dynamic(&a_grenade);
        self.dynamic_vec.push(a_grenade);
        self.grenades.push(grenade);
    }

    pub fn insert_character(&mut self, x: f64, y: f64, angle: f64) {
        let character = Rc::new(RefCell::new(Character::new(self.next_id(),x,y,angle)));
        let a_character = character.clone() as Rc<RefCell<BodyTrait>>;
        self.batch.insert_dynamic(&a_character);
        self.dynamic_vec.push(a_character);
        self.characters.push(character);
    }

    pub fn insert_boid(&mut self, x: f64, y: f64, angle: f64) {
        let boid = Rc::new(RefCell::new(Boid::new(self.next_id(),x,y,angle)));
        let a_boid = boid.clone() as Rc<RefCell<BodyTrait>>;
        self.batch.insert_dynamic(&a_boid);
        self.dynamic_vec.push(a_boid);
        self.boids.push(boid);
    }

    pub fn insert_moving_wall(&mut self, x: i32, y: i32, angle: Direction) {
        let moving_wall = Rc::new(RefCell::new(MovingWall::new(self.next_id(),x,y,angle,self.unit)));
        let a_moving_wall = moving_wall.clone() as Rc<RefCell<BodyTrait>>;
        self.batch.insert_dynamic(&a_moving_wall);
        self.dynamic_vec.push(a_moving_wall);
        self.moving_walls.push(moving_wall);
    }

    //pub fn insert_snake(&mut self, x: i32, y: i32, angle: Direction) {
    //    let snake = Rc::new(RefCell::new(Snake::new(self.next_id(),x,y,angle,self.unit,self.wall_map.clone(),self.batch.clone())));
    //    let a_snake = snake.clone() as Rc<BodyTrait>;
    //    self.batch.borrow_mut().insert_dynamic(&a_snake);
    //    self.dynamic_vec.push(a_snake);
    //    self.snakes.push(snake);
    //}

    //pub fn render(&mut self, _viewport: &Viewport, _camera: &Camera, _gl: &mut GlGraphics) {
    //}

    pub fn render(&mut self, frame_manager: &mut FrameManager, sound_manager: &mut SoundManager) {
        for w in &self.walls {
            w.borrow().render(frame_manager);
        }
        for a in &self.armories {
            a.borrow().render(frame_manager);
        }
        for mw in &self.moving_walls {
            mw.borrow_mut().render(frame_manager, sound_manager);
        }
        for b in &self.boids {
            b.borrow_mut().render(frame_manager, sound_manager);
        }
        for g in &self.grenades {
            g.borrow_mut().render(frame_manager, sound_manager);
        }
        for c in &self.characters {
            c.borrow_mut().render(frame_manager, sound_manager);
        }
    }

    pub fn update(&mut self, dt: f64) {
        for g in &self.grenades {
            g.update(dt,&self.batch);
        }
        let character_pos = [self.characters[0].borrow().x(),self.characters[0].borrow().y()];
        let to_create = boid_generator(self.boids.len(),character_pos,&self.wall_map,self.unit);
        for (x,y,a) in to_create {
            self.insert_boid(x,y,a);
        }
        for b in &self.boids {
            b.update(dt,character_pos,&self.boids);
        }
        for c in &self.characters {
            c.update(dt,&self.batch);
        }
        for mw in &self.moving_walls {
            mw.update(dt,&self.moving_walls,&self.wall_map);
        }

        // destroy dead bodies
        let mut i = 0;
        while i < self.dynamic_vec.len() {
            let b = self.dynamic_vec[i].borrow().dead();
            if b {
                let id = self.dynamic_vec[i].borrow().id();
                let body_type = self.dynamic_vec[i].borrow().body_type();
                match body_type {
                    BodyType::Boid => self.boids.retain(|b| {
                        b.borrow().id() != id
                    }),
                    BodyType::Grenade => self.grenades.retain(|b| {
                        b.borrow().id() != id
                    }),
                    _ => Err("detroy undetroyable body").unwrap(),
                }
                self.dynamic_vec.swap_remove(i);
            } else {
                i += 1;
            }
        }

        // resolve collisions
        {
            self.batch.clear_dynamic();
            for body in self.dynamic_vec.iter() {
                {
                    let body = &mut *body.borrow_mut();
                    let location = body.location();
                    let mut callback = |other: &mut BodyTrait| {
                        if body.collide(other) {
                            body.resolve_collision(other);
                            other.resolve_collision(body);
                            body.on_collision(other);
                            other.on_collision(body);
                        }
                    };
                    self.batch.apply_locally(&location,&mut callback);
                }
                self.batch.insert_dynamic(&(body.clone() as Rc<RefCell<BodyTrait>>));
            }
        }
    }

}

