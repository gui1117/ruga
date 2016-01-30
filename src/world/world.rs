use viewport::Viewport;
use opengl_graphics::GlGraphics;

use super::Camera;
use super::body::{ 
    Wall, 
    Character, 
    Snake,
    Monster, 
        Boid,
        Body,
        BodyTrait, 
};
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
    pub wall_map: Rc<RefCell<HashMap<[i32;2],bool>>>,
    pub walls: Vec<Rc<RefCell<Body>>>,
    pub monsters: Vec<Rc<RefCell<Body>>>,
    pub boids: Vec<Rc<RefCell<Boid>>>,
    pub snakes: Vec<Rc<RefCell<Snake>>>,
    pub characters: Vec<Rc<RefCell<Character>>>,
    pub static_vec: Vec<Rc<BodyTrait>>,
    pub dynamic_vec: Vec<Rc<BodyTrait>>,
    pub batch: Rc<RefCell<Batch>>,
}

impl World {
    pub fn new(unit: f64) -> World {
        World {
            unit: unit,
            time: 0.,
            next_id: 1,
            characters: Vec::new(),
            snakes: Vec::new(),
            monsters: Vec::new(),
            boids: Vec::new(),
            walls: Vec::new(),
            static_vec: Vec::new(),
            dynamic_vec: Vec::new(),
            batch: Rc::new(RefCell::new(Batch::new(unit))),
            wall_map: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn insert_wall(&mut self, x: i32, y: i32) {
        self.wall_map.borrow_mut().insert([x,y],true);

        let wall = Rc::new(RefCell::new(Wall::new(self.next_id(),x,y,self.unit)));
        let a_wall = wall.clone() as Rc<BodyTrait>;
        self.batch.borrow_mut().insert_static(&a_wall);
        self.static_vec.push(a_wall);
        self.walls.push(wall);
    }

    pub fn insert_character(&mut self, x: f64, y: f64, angle: f64) {
        let character = Rc::new(RefCell::new(Character::new(self.next_id(),x,y,angle,self.batch.clone())));
        let a_character = character.clone() as Rc<BodyTrait>;
        self.batch.borrow_mut().insert_dynamic(&a_character);
        self.dynamic_vec.push(a_character);
        self.characters.push(character);
    }

    pub fn insert_monster(&mut self, x: f64, y: f64, angle: f64) {
        let monster: Rc<RefCell<Body>> = Rc::new(RefCell::new(Monster::new(self.next_id(),x,y,angle)));
        let a_monster = monster.clone() as Rc<BodyTrait>;
        self.batch.borrow_mut().insert_dynamic(&a_monster);
        self.dynamic_vec.push(a_monster);
        self.monsters.push(monster);
    }

    pub fn insert_boid(&mut self, x: f64, y: f64, angle: f64) {
        let boid = Rc::new(RefCell::new(Boid::new(self.next_id(),x,y,angle,self.batch.clone())));
        let a_boid = boid.clone() as Rc<BodyTrait>;
        self.batch.borrow_mut().insert_dynamic(&a_boid);
        self.dynamic_vec.push(a_boid);
        self.boids.push(boid);
    }

    pub fn insert_snake(&mut self, x: i32, y: i32, angle: Direction) {
        let snake = Rc::new(RefCell::new(Snake::new(self.next_id(),x,y,angle,self.unit,self.wall_map.clone(),self.batch.clone())));
        let a_snake = snake.clone() as Rc<BodyTrait>;
        self.batch.borrow_mut().insert_dynamic(&a_snake);
        self.dynamic_vec.push(a_snake);
        self.snakes.push(snake);
    }

    pub fn render(&mut self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
        for b in self.static_vec.iter() {
            b.render(viewport,camera,gl);
        }
        for b in self.dynamic_vec.iter() {
            b.render(viewport,camera,gl);
        }
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
            radius: 1.,
            shape: LineShape::Round,
        };

        let mut lines = Vec::<[f64; 4]>::new();
        for b in self.static_vec.iter() {
            b.render_debug(&mut lines);
        }
        for b in self.dynamic_vec.iter() {
            b.render_debug(&mut lines);
        }

        gl.draw(*viewport, |context, gl| {
            let transform = camera.trans(context.transform);

            for line in lines {
                line_drawer.draw(line, default_draw_state(), transform, gl);
            }
        });
    }

    pub fn update(&mut self, dt: f64) {
        // update bodies
        for body in self.dynamic_vec.iter() {
            body.update(dt);
        }

        // resolve collisions
        {
            let mut batch = self.batch.borrow_mut();
            batch.clear_dynamic();
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
                    batch.apply_locally(&location,&mut callback);
                }
                batch.insert_dynamic(&(body.clone() as Rc<BodyTrait>));
            }
        }
    }

}

