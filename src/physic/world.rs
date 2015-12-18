use geometry::primitive::Rectangle;
use super::body::{ Body,BodySettings };
use std::collections::HashMap;

pub struct World {
	pub time: f64,
    pub bounds: Rectangle,
	//quadtree_max_object: usize, -> ask the quadtree if you want it
	//quadtree_deepness: usize,
	pub next_id: usize,
	pub bodies: HashMap<usize,Body>,
 //   pub event_receiver,
 //   pub body_update_receiver,
 //   pub possible_collision_receiver,
 //   pub final_update_receiver,
}

pub struct WorldBuilder {
    pub bounds: Option<Rectangle>,
    pub camera: Option<Camera>,
    pub bodies: Vec<BodySettings>,
}

#[derive(Clone,Debug)]
pub enum WorldBuilderError {
    Fail,
}

impl WorldBuilder {
    pub fn new() -> WorldBuilder {
        WorldBuilder {
            bounds: None,
            camera: None,
            bodies: vec![],
        }
    }

    pub fn build(self) -> Result<World,WorldBuilderError> {
        let bounds;
        if let Some(rect) = self.bounds {
            bounds = rect;
        } else {
            return Err(WorldBuilderError::Fail);
        }

        let mut bodies = HashMap::new();
        let mut next_id = 0;
        for setting in self.bodies {
            bodies.insert(next_id, Body::new(next_id,setting));
            next_id += 1;
        }

        Ok(World {
            time: 0.,
            bounds: bounds,
            bodies: bodies,
            next_id: next_id,
        })
    }

    pub fn bounds(mut self, rect: Rectangle) -> WorldBuilder {
        self.bounds = Some(rect);
        self
    }

    pub fn add_body(mut self, settings: BodySettings) -> WorldBuilder {
        self.bodies.push(settings);
        self
    }
}

impl World {
    pub fn update(&mut self, dt: f64) {
        //first parse event
        //  event from binary heap
        //  event from channel -> event_receiver
        //
        //second mutable must update themself in a thread
        //and return them to the receiver -> body_update_receiver
        //
        //the receiver send bodies to the collision possible detector
        //the collision detector return collision possible to a channel 
        //-> possible_collision_receiver 
        //
        //the receiver recreate thread to handle the collisions possible
        //those thread return a body 
        //
        //-> final_update_receiver
        //the receiver wait and write those body to the world when it's not use anymore
    }
}
