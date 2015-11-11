extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate collision_manager;

use piston::event_loop::Events;
use piston::input::{ 
	RenderArgs, 
	UpdateArgs, 
	RenderEvent, 
	UpdateEvent,
};

pub use geometry::Shape;
pub use character::Character;
pub use body::Body;
pub use collision_manager::{ Collidable, CollisionManager };

pub mod geometry;
pub mod body;
pub mod character;

pub struct App {
	gl: opengl_graphics::GlGraphics,
	rotation: f64,
}

impl App {
	fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0]; 

		let square = graphics::rectangle::square(0.0,0.0,50.0);
		let rotation = self.rotation;
		let (x,y) = ((args.width / 2) as f64, (args.height / 2) as f64);

		self.gl.draw(args.viewport(), |context, gl| {
			graphics::clear(GREEN, gl);

			let transform = context.transform.trans(x,y)
				.rot_rad(rotation)
				.trans(-25.0, -25.0);

			graphics::rectangle(RED, square, transform, gl);
		});
	}

	fn update(&mut self, args: &UpdateArgs) {
		self.rotation += 2.0 * args.dt;
	}
}

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
		rotation: 0.0
	};

	for e in window.events() {
		if let Some(r) = e.render_args() {
			app.render(&r);
		}
		if let Some(u) = e.update_args() {
			app.update(&u);
		}
	}
}
