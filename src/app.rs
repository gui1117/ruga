use api;
use specs;
use glium;
use glium::backend::Facade;
use graphics::{Graphics, Frame, Camera, Layer};
use collider::{self, Collider};
use collider::inter::DefaultInteractivity;

use std::thread;
use std::time::Duration;
use std::collections::HashMap;

const NUMBER_OF_THREADS: usize = 2;

#[derive(Clone)]
pub struct UpdateContext {
    pub dt: f32,
}

#[derive(Clone)]
pub struct CollideContext {
    pub id0: (specs::Entity, collider::HitboxId),
    pub id1: (specs::Entity, collider::HitboxId),
}

#[derive(Clone)]
pub struct SeparateContext {
    pub id0: (specs::Entity, collider::HitboxId),
    pub id1: (specs::Entity, collider::HitboxId),
}

pub type ColliderSpecsIdsMap = HashMap<collider::HitboxId, specs::Entity>;

pub struct App {
    must_quit: bool,
    graphics: Graphics,
    planner: specs::Planner<UpdateContext>,
}

impl App {
    pub fn new<F: Facade>(facade: &F) -> Self {
        let collider: Collider<DefaultInteractivity> = Collider::new(4.0, 0.01);

        let mut world = specs::World::new();
        world.add_resource(collider);
        world.add_resource(ColliderSpecsIdsMap::new());

        let planner = specs::Planner::new(world, NUMBER_OF_THREADS);

        App {
            graphics: Graphics::new(facade).unwrap(),
            must_quit: false,
            planner: planner,
        }
    }
    pub fn update(&mut self, dt: f32) {
        {
            let dt = dt as f64;

            let mut clock = 0.0;
            while clock < dt {
                let (event, id1, id2) = {
                    let mut collider = self.planner.mut_world().write_resource::<Collider>();
                    let timestep = collider.time_until_next().min(dt - clock);
                    clock += timestep;
                    collider.advance(timestep);
                    if let Some((event, id1, id2)) = collider.next() {
                        (event, id1, id2)
                    } else {
                        continue
                    }
                };
                // self.planner.run_custom(|r| {
                //     r.fetch(|_| {});
                //     thread::sleep(Duration::from_secs(2));
                //     println!("2");
                // });
                // self.planner.run_custom(|r| {
                //     r.fetch(|_| {});
                //     thread::sleep(Duration::from_secs(1));
                //     println!("1");
                // });
                // self.planner.wait();
                // println!("3");
            }
        }
        {
            let context = UpdateContext { dt: dt};
            self.planner.dispatch(context);
            self.planner.wait();
        }
    }
    pub fn draw(&mut self, frame: glium::Frame) {
        let camera = Camera::new(0.0, 0.0, 0.05);
        let mut frame = Frame::new(&mut self.graphics, frame, &camera);
        frame.finish().unwrap();
    }
    pub fn must_quit(&self) -> bool {
        self.must_quit
    }
}

impl api::Caller for App {
    fn quit(&mut self) {
        self.must_quit = true;
    }
    fn notify(&mut self, notification: String) {
        unimplemented!();
    }
}
