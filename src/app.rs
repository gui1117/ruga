use api;
use specs::{self, System};
use glium;
use glium::backend::Facade;
use graphics::{Graphics, Frame, Camera};
use collider::Collider;
use collider::inter::DefaultInteractivity;
use utils::AsSpecsId;
use systems;
use entities;

const NUMBER_OF_THREADS: usize = 2;
const NOTIFICATION_DURATION: usize = 600;

#[derive(Clone)]
pub struct UpdateContext {
    pub dt: f32,
}

#[derive(Clone)]
pub struct CollideContext {
    pub id0: specs::Entity,
    pub id1: specs::Entity,
}

#[derive(Clone)]
pub struct SeparateContext {
    pub id0: specs::Entity,
    pub id1: specs::Entity,
}

pub struct Notifications(pub Vec<(String, usize)>);

pub struct App {
    must_quit: bool,
    graphics: Graphics,
    planner: specs::Planner<UpdateContext>,
}

impl App {
    pub fn new<F: Facade>(facade: &F) -> Self {
        use components::*;

        let mut world = specs::World::new();
        world.register::<HitboxIdFlag>();
        world.register::<HitboxDraw>();
        world.register::<PlayerControl>();
        world.register::<CollisionBehavior>();

        let collider: Collider<DefaultInteractivity> = Collider::new(4.0, 0.01);
        world.add_resource(collider);
        world.add_resource(Notifications(Vec::new()));

        entities::add_character(&mut world, 0.0, 0.0);
        entities::add_wall(&mut world, 10.0, 1.0, 1.0, 1.0);

        let planner = specs::Planner::new(world, NUMBER_OF_THREADS);

        App {
            graphics: Graphics::new(facade).unwrap(),
            must_quit: false,
            planner: planner,
        }
    }
    pub fn update(&mut self, dt: f32) {
        let mut clock = 0.0;
        while clock < dt as f64 {
            use collider::Event::*;

            let (event, id0, id1) = {
                let mut collider = self.planner.mut_world().write_resource::<Collider>();
                let timestep = collider.time_until_next().min(dt as f64 - clock);
                clock += timestep;
                collider.advance(timestep);
                if let Some((event, id0, id1)) = collider.next() {
                    (event, id0, id1)
                } else {
                    continue
                }
            };

            match event {
                Collide => {
                    let context = CollideContext {
                        id0: id0.asi(),
                        id1: id1.asi(),
                    };
                    self.planner.run_custom(|run_arg| {
                        systems::ResolveCollision.run(run_arg, context);
                    });
                },
                Separate => {
                    let context = SeparateContext {
                        id0: id0.asi(),
                        id1: id1.asi(),
                    };
                },
            }
            self.planner.wait();
        }

        let context = UpdateContext { dt: dt};
        self.planner.dispatch(context);
        self.planner.wait();
    }
    pub fn draw(&mut self, frame: glium::Frame) {
        let camera = Camera::new(0.0, 0.0, 0.05);
        let mut frame = Frame::new(&mut self.graphics, frame, &camera);

        systems::draw_hitbox(self.planner.mut_world(), &mut frame);
        systems::draw_notifications(self.planner.mut_world(), &mut frame);

        frame.finish().unwrap();
    }
    pub fn must_quit(&self) -> bool {
        self.must_quit
    }
    pub fn resized(&mut self, _width: u32, _height: u32) {
        self.graphics.resize();
    }
}

impl api::Caller for App {
    fn quit(&mut self) {
        self.must_quit = true;
    }
    fn notify(&mut self, notification: String) {
        self.planner.mut_world().write_resource::<Notifications>().0.push((notification, NOTIFICATION_DURATION));
    }
    fn add_character(&mut self, x: f32, y: f32) {
        entities::add_character(self.planner.mut_world(), x, y);
    }
    fn add_wall(&mut self, x: f32, y: f32, w: f32, h: f32) {
        entities::add_wall(self.planner.mut_world(), x, y, w, h);
    }
}
