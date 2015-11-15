extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

use piston::event_loop::Events;
use piston::input::{ 
	RenderArgs, 
	UpdateArgs, 
	RenderEvent, 
	UpdateEvent,
	Event,
	Input,
};

use character::Character;
use world::World;

pub mod geometry;
pub mod body;
pub mod world;
pub mod character;
pub mod quadtree;

pub struct App {
	gl: opengl_graphics::GlGraphics,
	world: world::World,
}

impl App {
	fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

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

fn callback(world: &mut World) {
	println!("hello world !");
	world.add_event(0.1,CALL);
}

static CALL: &'static (Fn(&mut World)) = &callback;

fn main() {
	let opengl = opengl_graphics::OpenGL::V3_3;

	let window: glutin_window::GlutinWindow = piston::window::WindowSettings::new( "ruga", [640,480])
		.vsync(true)
		.opengl(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut app = App {
		gl: opengl_graphics::GlGraphics::new(opengl),
		world: world::World::new(0.,0.,100.,100.),
	};


	app.world.add_body(Character::new());
	app.world.add_event(0.1,CALL);

	for event in window.events() {
		match event {
			Event::Render(args) => app.render(&args),
			Event::Update(args) => app.update(&args),
			Event::AfterRender(_args) => (),
			Event::Idle(_args) => (),
			Event::Input(Input::Press(_button)) => (),
			Event::Input(Input::Release(_button)) => (),
			Event::Input(Input::Move(_motion)) => (),
			Event::Input(Input::Text(_text)) => (),
			Event::Input(Input::Resize(_width, _height)) => (),
			Event::Input(Input::Focus(_focus)) => (),
			Event::Input(Input::Cursor(_cursor)) => (),
		}
	}
}
