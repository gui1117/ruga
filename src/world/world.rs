use viewport::Viewport;
use opengl_graphics::GlGraphics;

use super::Camera;
use super::body::{ 
    Wall, 
    MovingWall, 
    Character, 
    Grenade,
    //Snake,
    Boid,
    Body,
    BodyTrait, 
};
use super::body::character::CharacterManager;
use super::body::moving_wall::MovingWallManager;
use super::body::grenade::GrenadeManager;
use super::body::boids::BoidManager;
use super::batch::Batch;
use util::direction::Direction;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct World {
    pub time: f64,
    pub unit: f64,
    next_id: usize,
    /// whether there is a wall or not
    pub wall_map: HashMap<[i32;2],bool>,
    pub walls: Vec<Rc<RefCell<Body>>>,
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
            //snakes: Vec::new(),
            boids: Vec::new(),
            grenades: Vec::new(),
            walls: Vec::new(),
            static_vec: Vec::new(),
            dynamic_vec: Vec::new(),
            batch: Batch::new(unit),
            wall_map: HashMap::new(),
        }
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn insert_wall(&mut self, x: i32, y: i32) {
        self.wall_map.insert([x,y],true);

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

    pub fn render(&mut self, _viewport: &Viewport, _camera: &Camera, _gl: &mut GlGraphics) {
    }

    pub fn render_debug(&mut self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
        use graphics::Transformed;
        use graphics::line::{ 
            Line as LineDrawer, 
            Shape as LineShape,
        };
        use graphics::default_draw_state;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.5]; 

        let line_drawer = LineDrawer {
            color: RED,
            radius: 0.2,
            shape: LineShape::Round,
        };

        let mut lines = Vec::<[f64; 4]>::new();
        for w in &self.walls {
            w.borrow().render_debug(&mut lines);
        }
        for mw in &self.moving_walls {
            mw.borrow().render_debug(&mut lines);
        }
        for b in &self.boids {
            b.borrow().render_debug(&mut lines);
        }
        for g in &self.grenades {
            g.borrow_mut().render_debug(&mut lines);
        }
        for c in &self.characters {
            c.render_debug(&mut lines);
        }

        gl.draw(*viewport, |context, gl| {
            let transform = camera.trans(context.transform);

            for line in lines {
                line_drawer.draw(line, default_draw_state(), transform, gl);
            }
        });
    }

    pub fn update(&mut self, dt: f64) {
        for g in &self.grenades {
            g.update(dt,&self.batch);
        }
        for b in &self.boids {
            b.update(dt,&self.batch);
        }
        for c in &self.characters {
            c.update(dt,&self.batch);
        }
        for mw in &self.moving_walls {
            mw.update(dt,&self.batch);
        }

        // destroy dead bodies
        let mut i = 0;
        while i < self.dynamic_vec.len() {
            let b = self.dynamic_vec[i].borrow().dead();
            if b {
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

