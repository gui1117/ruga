extern crate clap;
extern crate glium;
extern crate hlua;
extern crate time;
extern crate rustyline;

use glium::glutin;
use rustyline::Editor;
use rustyline::error::ReadlineError;

use std::str::FromStr;
use std::path::Path;
use std::time::Duration;
use std::thread;
use std::sync::mpsc::{channel, TryRecvError};
use std::sync::Mutex;
use std::sync::Arc;
use std::fs::File;
use std::io::{self, Write};

mod app;

static BILLION: u64 = 1_000_000_000;

static DEFAULT_CONFIG: &'static str = "
function poll_event()
end
";

enum API {
    Quit,
}

fn ns_to_duration(ns: u64) -> Duration {
    let secs = ns / BILLION;
    let nanos = (ns % BILLION) as u32;
    Duration::new(secs, nanos)
}

fn main() {
    let matches = clap::App::new("ruga")
        .version("0.3")
        .author("thiolliere <guillaume.thiolliere@opmbx.org>")
        .about("a game in rust")
        .arg(clap::Arg::with_name("vsync")
             .short("s")
             .long("vsync")
             .help("Set vsync"))
        .arg(clap::Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Set configuration file (lua)")
             .validator(|s| {
                 if Path::new(&*s).exists() {
                     Ok(())
                 } else {
                     Err(format!("configuration file '{}' doesn't exist",s))
                 }
             })
             .takes_value(true))
        .arg(clap::Arg::with_name("terminal")
             .short("t")
             .long("terminal")
             .help("Set lua terminal"))
        .arg(clap::Arg::with_name("dimension")
             .short("d")
             .long("dimensions")
             .value_name("DIMENSION")
             .help("Set dimensions (and unset fullscreen)")
             .validator(|s| {
                 u32::from_str(&*s)
                     .map(|_| ())
                     .map_err(|e| format!("'{}' dimension is invalid : {}", s, e))
             })
             .number_of_values(2)
             .takes_value(true))
        .arg(clap::Arg::with_name("fps")
             .short("f")
             .long("fps")
             .value_name("INT")
             .default_value("60")
             .validator(|s| {
                 u64::from_str(&*s)
                     .map(|_| ())
                     .map_err(|e| format!("'{}' fps is invalid : {}", s, e))
             })
             .help("Set multisampling")
             .takes_value(true))
        .arg(clap::Arg::with_name("multisampling")
             .short("m")
             .long("multisampling")
             .value_name("FACTOR")
             .possible_values(&["2", "4", "8", "16"])
             .help("Set multisampling")
             .takes_value(true))
        .get_matches();

    let window = {
        use glium::DisplayBuild;

        let mut builder = glutin::WindowBuilder::new();

        if matches.is_present("vsync") {
            builder = builder.with_vsync();
        }

        builder = match matches.value_of("factor") {
            Some("2") => builder.with_multisampling(2),
            Some("4") => builder.with_multisampling(4),
            Some("8") => builder.with_multisampling(8),
            Some("16") => builder.with_multisampling(16),
            Some(_) => unreachable!(),
            None => builder,
        };

        builder = if let Some(mut dimensions) = matches.values_of("dimension") {
            let width = u32::from_str(dimensions.next().unwrap()).unwrap();
            let height = u32::from_str(dimensions.next().unwrap()).unwrap();
            builder.with_dimensions(width, height)
        } else {
            builder.with_fullscreen(glutin::get_primary_monitor())
        };

        builder.build_glium().unwrap()
    };
    window.get_window().unwrap().set_cursor_state(glutin::CursorState::Hide).unwrap();

    let (api_tx, api_rx) = channel();

    let mut lua = hlua::Lua::new();
    lua.set("quit", hlua::function0(|| {
        api_tx.send(API::Quit).unwrap();
    }));

    lua.execute::<()>(DEFAULT_CONFIG).unwrap();
    if let Some(file) = matches.value_of("config") {
        lua.execute_from_reader::<(),_>(File::open(file).unwrap()).unwrap();
    }

    let lua = Arc::new(Mutex::new(lua));
    let terminal = if matches.is_present("terminal") {
        let lua_clone = lua.clone();
        let mut rl = Editor::<()>::new();
        Some(thread::spawn(move || {
            loop {
                let readline = rl.readline("> ");
                match readline {
                    Ok(line) => {
                        use hlua::LuaError::*;

                        rl.add_history_entry(&line);
                        match lua_clone.lock().unwrap().execute::<()>(&*line) {
                            Ok(()) => (),
                            Err(SyntaxError(s)) => println!("Syntax error: {}", s),
                            Err(ExecutionError(s)) => println!("Execution error: {}", s),
                            Err(ReadError(e)) => println!("Read error: {}", e),
                            Err(WrongType) => println!("Wrong type error: lua command must return nil"),
                        }
                    },
                    Err(ReadlineError::Interrupted) => {
                        api_tx.send(API::Quit).unwrap();
                        println!("^C");
                        break
                    },
                    Err(ReadlineError::Eof) => {
                        break
                    },
                    Err(err) => {
                        println!("Readline error: {:?}", err);
                    }
                }
            }
        }))
    } else {
        None
    };

    let mut app = app::App::new();
    let fps = u64::from_str(matches.value_of("fps").unwrap()).unwrap();
    let dt_ns = BILLION / fps;
    let dt = 1.0 / fps as f32;

    // game loop inspired by http://gameprogrammingpatterns.com/game-loop.html
    // and piston event loop
    //
    // if running out of time then slow down the game

    let mut last_time = time::precise_time_ns();

    'main_loop: loop {
        // poll events
        for event in window.poll_events() {
            use glium::glutin::Event::*;
            match event {
                Closed => break 'main_loop,
                _ => (),
            }
        }
        { lua.lock().unwrap().execute::<()>("poll_event()").unwrap(); }
        loop {
            match api_rx.try_recv() {
                Ok(API::Quit) => break 'main_loop,
                Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => break,
            }
        }

        // update
        app.update(dt);

        // draw
        app.draw();
        window.draw().finish().unwrap();

        let elapsed = time::precise_time_ns() - last_time;
        if elapsed < dt_ns {
            last_time = last_time + dt_ns;
            thread::sleep(ns_to_duration(dt_ns - elapsed));
        } else {
            last_time = time::precise_time_ns();
        }
    }

    if let Some(terminal) = terminal {
        // TODO draw explicit message
        // window.draw().finish().unwrap();
        // TODO coloring print
        print!("window has closed");
        io::stdout().flush().unwrap();
        terminal.join().unwrap();
    }
}
