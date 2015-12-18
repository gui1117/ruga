extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

use piston::event_loop::Events;
use piston::input::{ 
	RenderEvent, 
	UpdateEvent,
	Event,
	Input,
};

use world::World;
use world::body::character::Character;
use world::body::wall as Wall;
use world::geometry::Point;
use app::App;

pub mod input;
pub mod world;
pub mod direction;
pub mod app;
pub mod geometry;
pub mod physic;

fn main() {
	let opengl = opengl_graphics::OpenGL::V3_3;

	let window: glutin_window::GlutinWindow = piston::window::WindowSettings::new("ruga", [640,480])
		.vsync(true)
		.opengl(opengl)
		.exit_on_esc(false)
		.build()
		.unwrap();

	let mut app = App {
		gl: opengl_graphics::GlGraphics::new(opengl),
		world: world::World::new(0.,0.,500.,500.),
		quit: false,
		window_size: [640.,480.],
		player_id: None,
		player_dir: vec![],
	};



	app.world.add_body(Wall::new(vec![
						  Point {x:0.,y:0.},
						  Point {x:500.,y:0.},
						  Point {x:500.,y:10.},
						  Point {x:0.,y:10.}]));

	app.world.add_body(Wall::new(vec![
						  Point {x:0.,y:500.},
						  Point {x:500.,y:500.},
						  Point {x:500.,y:490.},
						  Point {x:0.,y:490.}]));

	app.world.add_body(Wall::new(vec![
						  Point {x:0.,y:500.},
						  Point {x:10.,y:500.},
						  Point {x:10.,y:0.},
						  Point {x:0.,y:0.}]));

	app.world.add_body(Wall::new(vec![
						  Point {x:500.,y:500.},
						  Point {x:490.,y:500.},
						  Point {x:490.,y:0.},
						  Point {x:500.,y:0.}]));

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


	World::load("map/map.tmx").unwrap();
	app.player_id = Some(app.world.add_body(Character::new(100.,100.)));

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
			Event::Input(Input::Move(motion)) => app.motion(&motion),
			Event::Input(Input::Text(_text)) => (),
			Event::Input(Input::Resize(_width, _height)) => (),
			Event::Input(Input::Focus(_focus)) => (),
			Event::Input(Input::Cursor(_cursor)) => (),
		}
	}
}
