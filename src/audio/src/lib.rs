#![warn(missing_docs)]

extern crate rustc_serialize;
extern crate rodio;

pub mod effect;
pub mod music;

use rodio::decoder::Decoder;
use rodio::Sink;

use std::sync::RwLock;
use std::path::{Path, PathBuf};
use std::fmt;
use std::fs::File;

use effect::DistanceModel;
use music::MusicTransition;

static mut RAW_STATE: *mut RwLock<State> = 0 as *mut RwLock<State>;

/// check at init if all music are OK
/// otherwise it may panic when playing the music
#[derive(Debug,Clone,Copy,PartialEq,RustcEncodable,RustcDecodable)]
pub enum CheckLevel {
    /// always check all music
    Always,
    /// check all music in debug mode only
    Debug,
    /// dont check music
    Never,
}

impl CheckLevel {
    fn check(&self) -> bool {
        match *self {
            CheckLevel::Always => true,
            CheckLevel::Never => false,
            CheckLevel::Debug => {
                let mut debug = false;
                debug_assert!({
                    debug = true;
                    true
                });
                debug
            }
        }
    }
}

#[derive(Clone,Debug,PartialEq,RustcEncodable,RustcDecodable)]
/// set musics, effects, volumes and audio player.
///
/// impl rustc_decodable and rustc_encodable
pub struct Setting {
    /// the base directory of effects
    pub effect_dir: PathBuf,

    /// the base directory of musics
    pub music_dir: PathBuf,

    /// global volume in [0,1]
    pub global_volume: f32,

    /// music volume in [0,1]
    pub music_volume: f32,

    /// effect volume in [0,1]
    pub effect_volume: f32,

    /// distance model for effect volume computation
    pub distance_model: DistanceModel,

    /// whereas the music must loop or not
    pub music_loop: bool,

    /// the kind of transition between musics
    pub music_transition: MusicTransition,

    /// the list of short effects
    ///
    /// each effect is identified by its position in the vector
    pub short_effect: Vec<PathBuf>,

    /// the list of persistent effects
    ///
    /// each effect is identified by its position in the vector
    pub persistent_effect: Vec<PathBuf>,

    /// the list of music
    ///
    /// each music is identified by its position in the vector
    pub music: Vec<PathBuf>,

    /// check level: always, debug or never
    pub check_level: CheckLevel,
}

/// set the global volume
pub fn set_volume(v: f32) {
    {
        let mut state = unsafe { (*RAW_STATE).write().unwrap() };
        state.global_volume = v;
    }
    update_volume();
}

fn update_volume() {
    effect::update_volume();
    music::update_volume();
}

/// return the global volume
pub fn volume() -> f32 {
    let state = unsafe { (*RAW_STATE).read().unwrap() };
    state.global_volume
}

/// stop music and effects
pub fn stop() {
    music::stop();
    effect::stop();
}

/// error possible on init
#[derive(Debug)]
pub enum InitError {
    /// baal has already been initialiazed
    DoubleInit,
}

impl fmt::Display for InitError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::InitError::*;
        match *self {
            DoubleInit => write!(fmt,"baal has already been initialized"),
        }
    }
}

/// init the audio player
pub fn init(setting: &Setting) -> Result<(), InitError> {
    unsafe {
        if !RAW_STATE.is_null() {
            return Err(InitError::DoubleInit);
        }

        let state = State::from_setting(setting);

        let box_state = Box::new(RwLock::new(state));
        RAW_STATE = Box::into_raw(box_state);

        Ok(())
    }
}

/// close the audio player, it can be init again.
pub fn close() {
    unsafe {
        if !RAW_STATE.is_null() {
            let mutex_state = Box::from_raw(RAW_STATE);
        }
        RAW_STATE = 0 as *mut RwLock<State>;
    }
}

/// reset audio from setting on the fly
/// it's OK if it hasn't been init
pub fn reset(setting: &Setting) -> Result<(),InitError> {
    unsafe {
        let old_raw_state = RAW_STATE;

        let state = State::from_setting(setting);

        let box_state = Box::new(RwLock::new(state));
        RAW_STATE = Box::into_raw(box_state);

        // drop old state
        if !old_raw_state.is_null() {
            let _ = Box::from_raw(old_raw_state);
        }

        Ok(())
    }
}

struct State {
    music_looping: bool,
    music_index: Option<usize>,
    music_transition: MusicTransition,
    listener: [f32;3],
    distance_model: DistanceModel,
    global_volume: f32,
    music_volume: f32,
    effect_volume: f32,
    music: Vec<PathBuf>,
    persistent_effect_positions: Vec<Vec<[f32;3]>>,
    persistent_mute: bool,
    short_effect: Vec<Decoder<File>>,
    music_sink: Sink,

}

impl State {
    fn from_setting(s: &Setting) -> State {
        unimplemented!();
        // let music_dir = Path::new(&s.music_dir);
        // let music: Vec<PathBuf> = s.music.iter().map(|name| music_dir.join(Path::new(&name))).collect();

        // State {
        //     music_looping: s.music_loop,
        //     music_index: None,
        //     music_transition: s.music_transition,
        //     listener: [0.,0.,0.],
        //     distance_model: s.distance_model.clone(),
        //     global_volume: s.global_volume,
        //     music_volume: s.music_volume,
        //     effect_volume: s.effect_volume,
        //     music: music,
        //     persistent_effect_positions: s.persistent_effect.iter().map(|_| vec!()).collect(),
        //     persistent_mute: false,
        // }
    }
}
