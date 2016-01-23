use viewport::Viewport;
use opengl_graphics::GlGraphics;
use world::{ 
    Camera, 
    WorldEvent, 
};

use super::{ 
    Body, 
    BodyTrait, 
    CollisionBehavior,
};
use world::event_heap::EventHeap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Character {
    body: Body,
    aim: f64,
    event_heap: Rc<RefCell<EventHeap<WorldEvent>>>,
}


impl Character {
    pub fn new(x: f64, y: f64, angle: f64, event_heap: Rc<RefCell<EventHeap<WorldEvent>>>) -> Character {
        Character {
            body: Body {
                id: 0,
                x: x,
                y: y,
                width2: 5.,
                height2: 5.,
                weight: 1.,
                velocity: 0.,
                angle: angle,
                mask: !0,
                group: 2,
                collision_behavior: CollisionBehavior::Persist,
            },
            aim: angle,
            event_heap: event_heap,
        }
    }

    pub fn aim(&self) -> f64 {
        self.aim
    }

    pub fn set_aim(&mut self, a: f64) {
        self.aim = a;
    }

    pub fn shoot(&mut self) {
    }
}

impl BodyTrait for Character {
    delegate!{
        body:
           id() -> usize,
           width2() -> f64,
           height2() -> f64,
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
           mut update(dt: f64) -> (),
           collision_behavior() -> CollisionBehavior,
           render(viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) -> (),
           mut on_collision(other: &BodyTrait) -> (),
    }
}
