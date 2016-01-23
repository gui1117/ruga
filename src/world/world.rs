use viewport::Viewport;
use opengl_graphics::GlGraphics;

use super::Camera;
use super::body::{ 
    Wall, 
    Character, 
    Monster, 
        Body,
        BodyTrait, 
};
use super::spatial_hashing::SpatialHashing;
use super::event_heap::EventHeap;

use util::{
    grid_raycast,
};

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashSet;

pub struct WorldEvent {
    pub callback: &'static Fn(&mut World, WorldEventArgs),
    pub args: WorldEventArgs,
}
pub enum WorldEventArgs {
    Nil,
}

pub struct World {
    pub time: f64,
    pub walls: Vec<Rc<RefCell<Body>>>,
    pub monsters: Vec<Rc<RefCell<Body>>>,
    pub characters: Vec<Rc<RefCell<Character>>>,
    pub static_vec: Vec<Rc<RefCell<BodyTrait>>>,
    pub dynamic_vec: Vec<Rc<RefCell<BodyTrait>>>,
    pub static_hashmap: SpatialHashing<Rc<RefCell<BodyTrait>>>,
    pub dynamic_hashmap: SpatialHashing<Rc<RefCell<BodyTrait>>>,
    pub events: Rc<RefCell<EventHeap<WorldEvent>>>,
}

impl World {
    pub fn new(unit: f64) -> World {
        World {
            time: 0.,
            characters: Vec::new(),
            monsters: Vec::new(),
            walls: Vec::new(),
            static_vec: Vec::new(),
            dynamic_vec: Vec::new(),
            static_hashmap: SpatialHashing::new(unit),
            dynamic_hashmap: SpatialHashing::new(unit),
            events: Rc::new(RefCell::new(EventHeap::new())),
        }
    }

    pub fn insert_wall(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let wall = Rc::new(RefCell::new(Wall::new(x,y,width,height)));
        let a_wall = wall.clone() as Rc<RefCell<BodyTrait>>;
        self.static_hashmap.insert(&wall.borrow().location(),&a_wall);
        self.static_vec.push(a_wall);
        self.walls.push(wall);
    }

    pub fn insert_character(&mut self, x: f64, y: f64, angle: f64) {
        let character = Rc::new(RefCell::new(Character::new(x,y,angle,self.events.clone())));
        let a_character = character.clone() as Rc<RefCell<BodyTrait>>;
        self.dynamic_vec.push(a_character);
        self.characters.push(character);
    }

    pub fn insert_monster(&mut self, x: f64, y: f64, angle: f64) {
        let monster: Rc<RefCell<Body>> = Rc::new(RefCell::new(Monster::new(x,y,angle)));
        let a_monster = monster.clone() as Rc<RefCell<BodyTrait>>;
        self.dynamic_vec.push(a_monster);
        self.monsters.push(monster);
    }

    pub fn render(&mut self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
        for b in self.static_vec.iter() {
            b.borrow().render(viewport,camera,gl);
        }
        for b in self.dynamic_vec.iter() {
            b.borrow().render(viewport,camera,gl);
        }
    }

    pub fn update(&mut self, dt: f64) {
        // parse event
        let mut vec = Vec::new();
        {
            let mut events = self.events.borrow_mut();
            while let Some(event) = events.pop() {
                vec.push(event);
            }
        }
        for WorldEvent{callback, args} in vec {
            callback(self,args);
        }

        // update bodies
        for body in self.dynamic_vec.iter() {
            body.borrow_mut().update(dt);
        }

        // resolve collisions
        self.dynamic_hashmap.clear();
        for body in self.dynamic_vec.iter() {
            {
                let mut body = body.borrow_mut();
                let location = body.location();
                let mut callback = |other: &Rc<RefCell<BodyTrait>>| {
                    let other = &mut *other.borrow_mut();
                    if body.collide(other) {
                        body.resolve_collision(other);
                        other.resolve_collision(&*body);
                    }
                };
                self.static_hashmap.apply_locally(&location,&mut callback);
                self.dynamic_hashmap.apply_locally(&location,&mut callback);
            }
            self.dynamic_hashmap.insert(&body.borrow().location(),&(body.clone() as Rc<RefCell<BodyTrait>>));
        }
    }

    /// callback return true when stop
    pub fn raycast<F: FnMut(&mut BodyTrait, f64, f64) -> bool>(&mut self, x: f64, y: f64, angle: f64, length: f64, callback: &mut F) {
        let unit = self.static_hashmap.unit();
        let x0 = x;
        let y0 = y;
        let x1 = x+length*angle.cos();
        let y1 = y+length*angle.sin();
        let index_vec = grid_raycast(x0/unit, y0/unit, x1/unit, y1/unit);

        // equation y = ax + b (we consider x0 and x1 never alined)
        let a = (y1 - y0)/(x1 - x0);
        let b = y0 -a*x0;

        let mut bodies: Vec<(Rc<RefCell<BodyTrait>>,f64,f64)>;
        let mut visited = HashSet::new();
        for i in &index_vec {
            let segment_start = (i[0] as f64)*unit;
            let segment_end = ((i[0]+1) as f64)*unit;
            bodies = Vec::new();

            let mut res = self.static_hashmap.get_on_index(i);
            res.append(&mut self.dynamic_hashmap.get_on_index(i));
            while let Some(body) = res.pop() {
                if !visited.contains(&body.borrow().id()) {
                    let op = body.borrow().raycast(a,b);
                    if let Some((x_min,y_min,x_max,y_max)) = op {
                        if segment_start < x_min && x_min < segment_end {
                            let min = ((x0-x_min).exp2() + (y0-y_min).exp2()).sqrt();
                            let max = ((x0-x_max).exp2() + (y0-y_max).exp2()).sqrt();
                            bodies.push((body,min,max));
                        }
                    }
                }
            }

            bodies.sort_by(|&(_,min_a,_),&(_,min_b,_)| {
                if min_a > min_b {
                    Ordering::Less
                } else if min_a == min_b {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            });

            for (body,min,max) in bodies {
                let body = &mut *body.borrow_mut();
                visited.insert(body.id());
                if callback(body,min,max) {
                    return;
                }
            }
        }
    }
}

