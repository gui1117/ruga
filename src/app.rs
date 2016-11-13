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

use std::rc::Rc;
use std::io::{self, Write};
use std::ops::Deref;

const NUMBER_OF_THREADS: usize = 2;
const NOTIFICATION_DURATION: usize = 600;
const NOTIFICATION_MAX: usize = 10;

#[derive(Clone)]
pub struct UpdateContext {
    pub dt: f32,
}

pub struct App {
    sensibility: f32,
    must_quit: bool,
    graphics: Graphics,
    planner: specs::Planner<UpdateContext>,
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
            sensibility: 1.,
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
        systems::draw_cursor(self.planner.mut_world(), &mut frame);
        systems::draw_physic(self.planner.mut_world(), &mut frame);

        frame.finish().unwrap();
    }
    pub fn must_quit(&self) -> bool {
        self.must_quit
    }
    pub fn resized(&mut self, _width: u32, _height: u32) {
        self.graphics.resize().unwrap();
    }
    pub fn move_cursor(&mut self, dx: f32, dy: f32, width: f32, height: f32) {
        let mut cursor = self.planner.mut_world().write_resource::<resources::Cursor>();
        cursor.x += dx * self.sensibility;
        cursor.y += dy * self.sensibility;

        let ratio = height / width;
        cursor.x = cursor.x.max(-1.).min(1.);
        cursor.y = cursor.y.max(-ratio).min(ratio);
    }
    pub fn cursor(&mut self) -> (f32, f32) {
        let cursor = self.planner.mut_world().read_resource::<resources::Cursor>();
        (cursor.x, cursor.y)
    }
}

impl_entity_builder!(App);

impl api::Caller for App {
    fn set_sensibility(&mut self, s: f32) {
        self.sensibility = s;
    }
    fn quit(&mut self) {
        self.must_quit = true;
    }
    fn notify(&mut self, notification: String) {
        let ref mut notifications =
            self.planner.mut_world().write_resource::<resources::Notifications>().0;
        notifications.push((notification, NOTIFICATION_DURATION));
        if notifications.len() > NOTIFICATION_MAX {
            notifications.remove(0);
        }
    }
    fn print(&mut self, msg: String) {
        print!("[{}]", msg);
        io::stdout().flush().unwrap();
    }
    fn fill_physic_world(&mut self) {
        let mut world = self.planner.mut_world();
        let mut physic_world = world.write_resource::<resources::PhysicWorld>();
        physic_world.fill(world);
    }
}
