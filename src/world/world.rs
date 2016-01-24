use viewport::Viewport;
use opengl_graphics::GlGraphics;

use super::Camera;
use super::body::{ 
    Wall, 
    Character, 
    Monster, 
        Boid,
        Body,
        BodyTrait, 
};
use super::spatial_hashing::SpatialHashing;
use super::batch::Batch;

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
    next_id: usize,
    pub walls: Vec<Rc<RefCell<Body>>>,
    pub monsters: Vec<Rc<RefCell<Body>>>,
    pub boids: Vec<Rc<RefCell<Boid>>>,
    pub characters: Vec<Rc<RefCell<Character>>>,
    pub static_vec: Vec<Rc<BodyTrait>>,
    pub dynamic_vec: Vec<Rc<BodyTrait>>,
    pub static_hashmap: SpatialHashing<Rc<BodyTrait>>,
    pub dynamic_hashmap: SpatialHashing<Rc<BodyTrait>>,
}

impl World {
    pub fn new(unit: f64) -> World {
        World {
            time: 0.,
            next_id: 1,
            characters: Vec::new(),
            monsters: Vec::new(),
            boids: Vec::new(),
            walls: Vec::new(),
            static_vec: Vec::new(),
            dynamic_vec: Vec::new(),
            static_hashmap: SpatialHashing::new(unit),
            dynamic_hashmap: SpatialHashing::new(unit),
        }
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn insert_wall(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let wall = Rc::new(RefCell::new(Wall::new(self.next_id(),x,y,width,height)));
        let a_wall = wall.clone() as Rc<BodyTrait>;
        self.static_hashmap.insert(&wall.location(),&a_wall);
        self.static_vec.push(a_wall);
        self.walls.push(wall);
    }

    pub fn insert_character(&mut self, x: f64, y: f64, angle: f64) {
        let character = Rc::new(RefCell::new(Character::new(self.next_id(),x,y,angle)));
        let a_character = character.clone() as Rc<BodyTrait>;
        self.dynamic_vec.push(a_character);
        self.characters.push(character);
    }

    pub fn insert_monster(&mut self, x: f64, y: f64, angle: f64) {
        let monster: Rc<RefCell<Body>> = Rc::new(RefCell::new(Monster::new(self.next_id(),x,y,angle)));
        let a_monster = monster.clone() as Rc<BodyTrait>;
        self.dynamic_vec.push(a_monster);
        self.monsters.push(monster);
    }

    pub fn insert_boid(&mut self, x: f64, y: f64, angle: f64) {
        let boid = Rc::new(RefCell::new(Boid::new(self.next_id(),x,y,angle)));
        let a_boid = boid.clone() as Rc<BodyTrait>;
        self.dynamic_vec.push(a_boid);
        self.boids.push(boid);
    }

    pub fn render(&mut self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
        for b in self.static_vec.iter() {
            b.render(viewport,camera,gl);
        }
        for b in self.dynamic_vec.iter() {
            b.render(viewport,camera,gl);
        }
    }

    pub fn update(&mut self, dt: f64) {
        // update bodies
        {
            let batch = Batch::<Rc<BodyTrait>>::new(&self.static_hashmap,&self.dynamic_hashmap);
            for body in self.dynamic_vec.iter() {
                body.update(dt,&batch);
            }
        }

        // resolve collisions
        self.dynamic_hashmap.clear();
        for body in self.dynamic_vec.iter() {
            {
                let location = body.location();
                let mut callback = |other: &Rc<BodyTrait>| {
                    let other = &**other;
                    if body.collide(other) {
                        body.resolve_collision(other);
                        other.resolve_collision(&**body);
                        body.on_collision(other);
                        other.on_collision(&**body);
                    }
                };
                self.static_hashmap.apply_locally(&location,&mut callback);
                self.dynamic_hashmap.apply_locally(&location,&mut callback);
            }
            self.dynamic_hashmap.insert(&body.location(),&(body.clone() as Rc<BodyTrait>));
        }
    }

    /// callback return true when stop
    pub fn raycast<F: FnMut(&BodyTrait, f64, f64) -> bool>(&mut self, x: f64, y: f64, angle: f64, length: f64, callback: &mut F) {
        let unit = self.static_hashmap.unit();
        let x0 = x;
        let y0 = y;
        let x1 = x+length*angle.cos();
        let y1 = y+length*angle.sin();
        let index_vec = grid_raycast(x0/unit, y0/unit, x1/unit, y1/unit);

        // equation y = ax + b (we consider x0 and x1 never alined)
        let a = (y1 - y0)/(x1 - x0);
        let b = y0 -a*x0;

        let mut bodies: Vec<(Rc<BodyTrait>,f64,f64)>;
        let mut visited = HashSet::new();
        for i in &index_vec {
            let segment_start = (i[0] as f64)*unit;
            let segment_end = ((i[0]+1) as f64)*unit;
            bodies = Vec::new();

            let mut res = self.static_hashmap.get_on_index(i);
            res.append(&mut self.dynamic_hashmap.get_on_index(i));
            while let Some(body) = res.pop() {
                if !visited.contains(&body.id()) {
                    let op = body.raycast(a,b);
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
                let body = &*body;
                visited.insert(body.id());
                if callback(body,min,max) {
                    return;
                }
            }
        }
    }
}

