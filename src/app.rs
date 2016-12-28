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
use weapon;

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
    must_quit: bool,
    graphics: Graphics,
    planner: specs::Planner<UpdateContext>,
}

impl App {
    pub fn new<F: Facade>(facade: &F) -> Self {
        use components::*;

        let mut world = specs::World::new();

        resources::add_resources(&mut world);
        components::register_components(&mut world);

        let mut planner = specs::Planner::new(world, NUMBER_OF_THREADS);
        systems::update::add_systems(&mut planner);

        App {
            graphics: Graphics::new(facade).unwrap(),
            must_quit: false,
            planner: planner,
        }
    }
    pub fn update(&mut self, dt: f32) {
        let context = UpdateContext { dt: dt };
        self.planner.dispatch(context);
        self.planner.wait();
    }
    pub fn draw(&mut self, frame: glium::Frame) {
        let camera = {
            let mut world = self.planner.mut_world();
            let players = world.read::<components::PlayerControl>();
            let states = world.read::<components::PhysicState>();
            let zoom = world.read_resource::<resources::Zoom>().0;

            let mut pos = [0.,0.];
            for (_, state) in (&players, &states).iter() {
                pos =  state.pos;
            }
            Camera::new(pos[0], pos[1], zoom)
        };
        let mut frame = Frame::new(&mut self.graphics, frame, &camera);
        systems::draw::run(self.planner.mut_world(), &mut frame);
        frame.finish().unwrap();
    }
    pub fn must_quit(&self) -> bool {
        self.must_quit
    }
    pub fn resized(&mut self, _width: u32, _height: u32) {
        self.graphics.resize().unwrap();
    }
    pub fn set_cursor(&mut self, x: f32, y: f32) {
        let mut cursor = self.planner.mut_world().write_resource::<resources::Cursor>();
        cursor.x = x;
        cursor.y = y;
    }
    pub fn cursor(&mut self) -> (f32, f32) {
        let cursor = self.planner.mut_world().read_resource::<resources::Cursor>();
        (cursor.x, cursor.y)
    }
}

impl_entity_builder!(App);

impl api::Caller for App {
    fn set_player_aim(&mut self, angle: f32) {
        let mut world = self.planner.mut_world();
        let players = world.read::<components::PlayerControl>();
        let mut aims = world.write::<components::Aim>();
        for (_, mut aim) in (&players, &mut aims).iter() {
            aim.0 = angle;
        }
    }
    fn set_player_force(&mut self, angle: f32, strength: f32) {
        let mut world = self.planner.mut_world();
        let players = world.read::<components::PlayerControl>();
        let mut forces = world.write::<components::PhysicForce>();
        for (_, force) in (&players, &mut forces).iter() {
            force.angle = angle;
            force.strength = strength;
        }
    }
    fn quit(&mut self) {
        self.must_quit = true;
    }
    fn notify(&mut self, notification: String) {
        let mut world = self.planner.mut_world();
        let ref mut notifications = world.write_resource::<resources::Notifications>().0;
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
    fn set_zoom(&mut self, new_zoom: f32) {
        let mut world = self.planner.mut_world();
        let ref mut zoom = world.write_resource::<resources::Zoom>().0;
        *zoom = new_zoom;
    }
    fn set_player_shoot(&mut self, shoot: bool) {
        let mut world = self.planner.mut_world();
        let mut shoots = world.write::<components::Shoot>();
        let players = world.read::<components::PlayerControl>();

        for (_, entity) in (&players, &world.entities()).iter() {
            if shoot {
                shoots.insert(entity, components::Shoot);
            } else {
                shoots.remove(entity);
            }
        }
    }
    fn set_player_weapon(&mut self, kind: String, reload: f32, setup: f32, setdown: f32) {
        if let Some(kind) = weapon::Kind::from_str(&*kind) {
            let next_weapon = components::NextWeapon(components::Weapon {
                kind: kind,
                state: weapon::State::Setup(0.),
                reload_factor: 1./reload,
                setup_factor: 1./setup,
                setdown_factor: 1./setdown,
            });

            let mut world = self.planner.mut_world();
            let mut next_weapons = world.write::<components::NextWeapon>();
            let players = world.read::<components::PlayerControl>();

            for (_, entity) in (&players, &world.entities()).iter() {
                next_weapons.insert(entity, next_weapon.clone());
            }
        }
    }
}
