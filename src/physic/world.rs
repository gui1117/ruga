use super::geometry::primitive::Rectangle;
use std::collections::HashMap;
use super::quadtree::{ Identifiable, Localisable };
use super::viewport::Viewport;
use super::camera::Camera;
use super::opengl_graphics::GlGraphics;
use std::thread;
use std::sync::mpsc::{ channel, Sender, Receiver };

pub trait Body: Sized + Sync + Send + Identifiable + Localisable {
    type Settings;
    //type Delta;
    //type Collision;
    fn new(usize,Self::Settings) -> Self;
	fn render_debug(&self, viewport: Viewport, camera: &Camera, gl: &mut GlGraphics);
    fn aware_update(&self, dt: f64, bodies: &Vec<Self>) -> Self;
    fn oblivious_update(&self, dt: f64) -> Self;
}

pub struct World<B: Body> {
	pub time: f64,
    pub bounds: Rectangle,
	//quadtree_max_object: usize, -> ask the quadtree if you want it
	//quadtree_deepness: usize,
	pub next_id: usize,
	pub bodies: HashMap<usize,B>,//TODO
    //pub event_receiver,
    pub body_update_receiver: Receiver<B>,
    pub body_update_sender: Sender<B>,
    pub possible_collision_receiver: Receiver<(B,B)>,
    pub possible_collision_sender: Sender<(B,B)>,
    pub final_update_receiver: Receiver<B>,
    pub final_update_sender: Sender<B>,
}

pub struct WorldBuilder<B: Body> {
    pub bounds: Option<Rectangle>,
    pub bodies: Vec<B::Settings>,
}

#[derive(Clone,Debug)]
pub enum WorldBuilderError {
    Fail,
}

impl<B: Body> WorldBuilder<B> {
    pub fn new() -> WorldBuilder<B> {
        WorldBuilder {
            bounds: None,
            bodies: vec![],
        }
    }

    pub fn build(self) -> Result<World<B>,WorldBuilderError> {
        let bounds;
        if let Some(rect) = self.bounds {
            bounds = rect;
        } else {
            return Err(WorldBuilderError::Fail);
        }

        let mut bodies = HashMap::new();
        let mut next_id = 0;
        for setting in self.bodies {
            bodies.insert(next_id, B::new(next_id,setting));
            next_id += 1;
        }

        let (body_update_sender,body_update_receiver) = channel();
        let (possible_collision_sender,possible_collision_receiver) = channel();
        let (final_update_sender,final_update_receiver) = channel();

        Ok(World::<B> {
            time: 0.,
            bounds: bounds,
            bodies: bodies,
            next_id: next_id,

            body_update_receiver: body_update_receiver,
            body_update_sender: body_update_sender,

            possible_collision_receiver: possible_collision_receiver,
            possible_collision_sender: possible_collision_sender,

            final_update_receiver: final_update_receiver,
            final_update_sender: final_update_sender,
        })
    }

    pub fn bounds(mut self, rect: Rectangle) -> WorldBuilder<B> {
        self.bounds = Some(rect);
        self
    }

    pub fn add_body(mut self, settings: B::Settings) -> WorldBuilder<B> {
        self.bodies.push(settings);
        self
    }
}

impl<B: Body> World<B> {
    pub fn render_debug(&self, viewport: Viewport, camera: &Camera, gl: &mut GlGraphics) {
        for (_,body) in &self.bodies {
            body.render_debug(viewport,camera,gl);
        }
    }

    pub fn update(&mut self, dt: f64) {
        //first parse event
        //  event from binary heap
        //  event from channel -> event_receiver
        //
        //second mutable must update themself in a thread T:update(&mut self,dt)
        //and return them to the receiver -> body_update_receiver
        //
        //the receiver send bodies to the collision possible detector T:localizable,identifiable,
        //the collision detector return collision possible to a channel 
        //-> possible_collision_receiver 
        //
        //the receiver recreate thread to handle the collisions possible 
        //  T:overlap(T')
        //  T:collision
        //those thread return a body 
        //
        //-> final_update_receiver
        //the receiver wait and write those body to the world when it's not use anymore
        //and compute the final quadtree
    }
}
