use api;
use specs::{self, System};
use glium;
use glium::backend::Facade;
use graphics::{Graphics, Frame, Camera};
use specs::Join;
use systems;
use components;
use resources;
use entities;

use std::io::{self, Write};

const NUMBER_OF_THREADS: usize = 2;
const NOTIFICATION_DURATION: usize = 600;
const NOTIFICATION_MAX: usize = 10;

#[derive(Clone)]
pub struct UpdateContext {
    pub dt: f32,
}

pub struct App {
    must_quit: bool,
    graphics: Graphics,
    planner: specs::Planner<UpdateContext>,
    debug: Option<((f32,f32),(f32,f32),(f32,f32),(f32,f32))>,
}

impl App {
    pub fn new<F: Facade>(facade: &F) -> Self {
        use components::*;

        let mut world = specs::World::new();

        resources::add_resource(&mut world);
        components::register_components(&mut world);

        let planner = specs::Planner::new(world, NUMBER_OF_THREADS);

        App {
            graphics: Graphics::new(facade).unwrap(),
            must_quit: false,
            planner: planner,
            debug: None,
        }
    }
    pub fn update(&mut self, dt: f32) {
        // TODO Physic world update
        let context = UpdateContext { dt: dt };
        self.planner.dispatch(context);
        self.planner.wait();
    }
    pub fn draw(&mut self, frame: glium::Frame) {
        let camera = Camera::new(0.0, 0.0, 0.05);
        let mut frame = Frame::new(&mut self.graphics, frame, &camera);

        systems::draw_notifications(self.planner.mut_world(), &mut frame);
        systems::draw_debug(self.planner.mut_world(), &mut frame);
        if let Some((p1,p2,p3,p4)) = self.debug {
            frame.draw_line(p1, p2, p3, p4, 0.1, ::graphics::Layer::Middle, ::colors::GREEN);
        }

        frame.finish().unwrap();
    }
    pub fn must_quit(&self) -> bool {
        self.must_quit
    }
    pub fn resized(&mut self, _width: u32, _height: u32) {
        self.graphics.resize().unwrap();
    }
}

impl api::Caller for App {
    fn quit(&mut self) {
        self.must_quit = true;
    }
    fn notify(&mut self, notification: String) {
        let ref mut notifications = self.planner.mut_world().write_resource::<resources::Notifications>().0;
        notifications.push((notification, NOTIFICATION_DURATION));
        if notifications.len() > NOTIFICATION_MAX {
            notifications.remove(0);
        }
    }
    fn print(&mut self, msg: String) {
        print!("[{}]", msg);
        io::stdout().flush().unwrap();
    }
    fn debug_raycast(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.debug = Some(((x1,y1),(x2,y2),(x1,y1),(x2,y2)));
        let mut world = self.planner.mut_world();
        let mut debug_actives = world.write::<components::DebugActive>();
        for mut active in (&mut debug_actives).iter() {
            active.active = false;
        }
        let physic_world = world.read_resource::<resources::PhysicWorld>();
        let ray = ::physics::RayCast {
            origin: [x1,y1],
            angle: (y2-y1).atan2(x2-x1),
            length: ((y2-y1).powi(2) + (x2-x1).powi(2)).sqrt(),
            mask: !0,
            group: !0,
        };
        physic_world.raycast(&ray, &mut |(entity,_,_)| {
            if let Some(active) = debug_actives.get_mut(entity.entity) {
                active.active = true;
            }
            ::physics::ContinueOrStop::Continue
        });
    }
    fn add_debug_rectangle(&mut self, x: f32, y: f32, w: f32, h: f32) {
        entities::add_debug_rectangle(self.planner.mut_world(), x, y, w, h);
    }
    fn add_debug_circle(&mut self, x: f32, y: f32, r: f32) {
        entities::add_debug_circle(self.planner.mut_world(), x, y, r);
    }
    fn fill_physic_world(&mut self) {
        let mut world = self.planner.mut_world();
        let mut physic_world = world.write_resource::<resources::PhysicWorld>();
        physic_world.fill(world);
    }
}
