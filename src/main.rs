extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

use std::fmt;

use piston::event_loop::Events;
use piston::input::{ 
	RenderArgs, 
	UpdateArgs, 
	RenderEvent, 
	UpdateEvent,
	Event,
	Input,
};

use body::character::Character;
use body::wall as Wall;
use world::World;
use geometry::Point;

pub mod weapon;
pub mod input;
pub mod geometry;
pub mod body;
pub mod world;
pub mod camera;
pub mod collision_manager;

enum Direction {
	Left,
	Right,
	Up,
	Down,
}

impl Direction {
	pub fn perpendicular(&self, other: &Direction) -> bool {
		match self {
			&Direction::Up | &Direction::Down => {
				match other {
					&Direction::Right | &Direction::Left => true,
					_ => false,
				}
			},

			&Direction::Right | &Direction::Left => {
				match other {
					&Direction::Up | &Direction::Down => true,
					_ => false,
				}
			},
		}
	}
}

impl fmt::Debug for Direction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Direction::Up => write!(f,"Up"),
			&Direction::Down => write!(f,"Down"),
			&Direction::Left => write!(f,"Left"),
			&Direction::Right => write!(f,"Right"),
		}
	}
}

pub struct App {
	gl: opengl_graphics::GlGraphics,
	world: world::World,
	pub quit: bool,
	character_id: Option<usize>,
	character_dir: Vec<Direction>,
}

impl App {
	fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

		self.world.update_camera(args,self.character_id);

		const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

		self.gl.draw(args.viewport(), |_, gl| {
			graphics::clear(BLACK, gl);
		});

		self.world.render_debug(args,&mut self.gl);
	}

	fn update(&mut self, args: &UpdateArgs) {
		self.world.update(args.dt);
	}
}

fn main() {
	let opengl = opengl_graphics::OpenGL::V3_3;

	let window: glutin_window::GlutinWindow = piston::window::WindowSettings::new( "ruga", [640,480])
		.vsync(true)
		.opengl(opengl)
		.exit_on_esc(false)
		.build()
		.unwrap();

	let mut app = App {
		gl: opengl_graphics::GlGraphics::new(opengl),
		world: world::World::new(0.,0.,500.,500.),
		quit: false,
		character_id: None,
		character_dir: vec![],
	};


	app.world.add_body(Wall::new(vec![
						  Point {x:20.,y:20.},
						  Point {x:80.,y:20.},
						  Point {x:80.,y:80.},
						  Point {x:20.,y:80.}]));

	app.world.add_body(Wall::new(vec![
						  Point {x:20.,y:80.},
						  Point {x:80.,y:80.},
						  Point {x:80.,y:160.},
						  Point {x:20.,y:160.}]));

	app.character_id = Some(app.world.add_body(Character::new()));


	for event in window.events() {
		if app.quit {
			return;
		}

		match event {
			Event::Render(args) => app.render(&args),
			Event::Update(args) => app.update(&args),
			Event::AfterRender(_args) => (),
			Event::Idle(_args) => (),
			Event::Input(Input::Press(button)) => app.press(&button),
			Event::Input(Input::Release(button)) => app.release(&button),
			Event::Input(Input::Move(_motion)) => (),
			Event::Input(Input::Text(_text)) => (),
			Event::Input(Input::Resize(_width, _height)) => (),
			Event::Input(Input::Focus(_focus)) => (),
			Event::Input(Input::Cursor(_cursor)) => (),
		}
	}
}
