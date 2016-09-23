#[macro_use] extern crate configuration;
#[macro_use] extern crate lazy_static;
extern crate baal;
extern crate graphics;
extern crate glium;
extern crate specs;
extern crate time;
extern crate toml;
extern crate rand;
extern crate fnv;
extern crate png;
extern crate gilrs;

mod persistent_snd;
mod levels;
mod app;
mod conf;
mod event_loop;
mod control;
mod physic;
mod entities;
mod utils;
mod life;
mod portal;
mod text;

mod components {
    pub use control::{
        PlayerControl,
        TowardPlayerControl,
        MonsterControl,
    };
    pub use physic::{
        GridSquare,
        PhysicState,
        PhysicType,
        PhysicForce,
        PhysicDynamic,
        PhysicStatic,
        PhysicTrigger,
        Shape,
        Ray,
        CollisionBehavior,
    };
    pub use life::{
        Column,
        Life,
        Killer,
        Ball,
    };
    pub use portal::Portal;
    pub use app::Graphic;
    pub use text::{
        FixedCameraText,
        FixedCamera,
        Text,
    };
    pub use persistent_snd::DynPersistentSnd;
}
mod resource {
    pub use physic::PhysicWorld;
}
mod systems {
    pub use physic::PhysicSystem;
    pub use life::{
        LifeSystem,
        KillerSystem,
        BallSystem,
        ColumnSystem,
    };
    pub use control::{
        PlayerSystem,
        MonsterSystem,
        TowardPlayerSystem,
    };
    pub use portal::PortalSystem;
    pub use persistent_snd::PersistentSndSystem;
}

pub use conf::CONFIG as config;

use glium::glutin;
use std::time::Duration;
use std::thread;
use event_loop::{
    Events,
    Event,
};

fn init() -> Result<(app::App,glium::backend::glutin_backend::GlutinFacade,event_loop::WindowEvents,gilrs::Gilrs),String> {
    use glium::DisplayBuild;

    let mut musics = vec!();
    musics.push(config.levels.entry_music.clone());

    // load casltes
    let (castles,mut musics) = try!(levels::load_castles(musics).map_err(|e| format!("ERROR: levels castles load failed: {}",e)));

    // init baal
    try!(baal::init(&baal::Setting {
        channels: config.audio.channels,
        sample_rate: config.audio.sample_rate,
        frames_per_buffer: config.audio.frames_per_buffer,
        effect_dir: config.audio.effect_dir.clone().into(),
        music_dir: config.audio.music_dir.clone().into(),
        global_volume: config.audio.global_volume,
        music_volume: config.audio.music_volume,
        effect_volume: config.audio.effect_volume,
        distance_model: match &*config.audio.distance_model {
            "linear" => baal::effect::DistanceModel::Linear(config.audio.distance_model_min,config.audio.distance_model_max),
            "pow2" => baal::effect::DistanceModel::Pow2(config.audio.distance_model_min,config.audio.distance_model_max),
            _ => unreachable!(),
        },
        music_loop: config.audio.music_loop,
        short_effect: config.audio.short_effects.iter().cloned().map(|n| n.into()).collect(),
        persistent_effect: config.audio.persistent_effects.iter().cloned().map(|n| n.into()).collect(),
        music: musics.drain(..).map(|music| music.into()).collect(),
        check_level: match &*config.audio.check_level {
            "never" => baal::CheckLevel::Never,
            "always" => baal::CheckLevel::Always,
            "debug" => baal::CheckLevel::Debug,
            _ => unreachable!(),
        },
        music_transition: match &*config.audio.transition_type {
            "instant" => baal::music::MusicTransition::Instant,
            "smooth" => baal::music::MusicTransition::Smooth(config.audio.transition_time),
            "overlap" => baal::music::MusicTransition::Overlap(config.audio.transition_time),
            _ => unreachable!(),
        },
    }).map_err(|e| format!("ERROR: audio init failed: {}",e)));

    // init window
    // TODO if fail then disable vsync and then multisampling and then vsync and multisamping
    let window = {
        let mut builder = glium::glutin::WindowBuilder::new();

        if config.window.vsync {
            builder = builder.with_vsync();
        }
        if config.window.multisampling != 0 {
            builder = builder.with_multisampling(config.window.multisampling);
        }
        if config.window.fullscreen {
            if config.window.fullscreen_on_primary_monitor {
                builder = builder.with_fullscreen(glutin::get_primary_monitor());
            } else {
                builder = builder.with_fullscreen(try!(glutin::get_available_monitors().nth(config.window.fullscreen_monitor)
                                                  .ok_or("ERROR: window init failed: fullsceen monitor specified unavailable")));
            }
        } else {
            builder = builder.with_dimensions(config.window.dimension[0], config.window.dimension[1])
                .with_title(format!("ruga"));
        }
        try!(builder.build_glium().map_err(|e| format!("ERROR: window init failed: {}",e)))
    };
    window.get_window().unwrap().set_cursor_state(glium::glutin::CursorState::Hide).unwrap();

    // init app
    let app = try!(app::App::new(&window,castles).map_err(|e| format!("ERROR: app creation failed: {}",e)));

    // init event loop
    let window_events = window.events(&event_loop::Setting {
        ups: config.event_loop.ups,
        max_fps: config.event_loop.max_fps,
    });


    Ok((app,window,window_events,gilrs::Gilrs::new()))
}

fn main() {
    // init
    let (mut app,mut window,mut window_events, mut gamepad) = match init() {
        Ok(t) => t,
        Err(err) => {
            println!("{}",err);
            std::process::exit(1);
        },
    };

    // game loop
    while let Some(event) = window_events.next(&mut window, &mut gamepad) {
        match event {
            Event::Update(args) => app.update(args),
            Event::Render(args) => app.render(args),
            Event::GlutinEvent(glutin::Event::Closed) => break,
            Event::GlutinEvent(glutin::Event::KeyboardInput(state,keycode,_)) => {
                if state == glutin::ElementState::Pressed {
                    app.key_pressed(keycode);
                } else {
                    app.key_released(keycode);
                }
            },
            Event::GlutinEvent(glutin::Event::Resized(width,height)) => app.resize(width,height),
            Event::GlutinEvent(glutin::Event::Focused(f)) => app.focused(f),
            Event::GlutinEvent(glutin::Event::Touch(t)) => app.touch(t),
            Event::GlutinEvent(_) => (),
            Event::GilrsEvent(gilrs::Event::ButtonPressed(button)) => app.button_pressed(button),
            Event::GilrsEvent(gilrs::Event::ButtonReleased(button)) => app.button_released(button),
            Event::GilrsEvent(gilrs::Event::AxisChanged(axis,pos)) => app.axis_changed(axis,pos),
            Event::GilrsEvent(_) => (),
            Event::Idle(args) => thread::sleep(Duration::from_millis(args.dt as u64)),
        }

        if app.quit {
            baal::close();
            return;
        }
    }
}

#[test]
fn main_test() {
    if let Err(err) = init() {
        println!("{}",err);
        std::process::exit(1);
    }
}
