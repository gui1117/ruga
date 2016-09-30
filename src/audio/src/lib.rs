#![warn(missing_docs)]

extern crate rustc_serialize;
extern crate rodio;

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

pub mod effect {
    //! this module allow to play short and persistent sound effects
    //!
    //! be careful that `set_volume`, `set_listener`, `set_distance_model`
    //! only affect future short sound effects

    use super::RAW_STATE;

    fn update_volume() {
        short::update_volume();
        persistent::update_volume_for_all();
    }

    /// set the volume of sound effects
    /// take effect for future sounds effects only
    pub fn set_volume(v: f32) {
        let mut state = unsafe { (*RAW_STATE).write().unwrap() };
        state.effect_volume = v;
        update_volume();
    }

    /// return the volume of sound effects
    pub fn volume() -> f32 {
        let state = unsafe { (*RAW_STATE).read().unwrap() };
        state.effect_volume
    }

    pub mod short {
        //! this module allow to play short sound effects
        //!
        //! ```lua
        //! volume = global_volume * effect_volume * distance(position,listener_position)
        //! ```
        //!
        //! but once a sound effect is played at a volume it doesn't change its volume anymore
        //!
        //! this can lead to weird effects for not so short sound effects and with moving source

        use super::super::{RAW_STATE};

        /// play the sound effect at the volume: `global_volume * effect_volume *
        /// distance(position, listener_position)`
        pub fn play(effect: usize, pos: [f32;3]) {
            let state = unsafe { (*RAW_STATE).read().unwrap() };
            let volume = state.global_volume * state.effect_volume * state.distance_model.distance(pos,state.listener);
            if volume > 0. {
                Sink::new(state.endpoint).append(state.short_effect(effect).iter())
                //TODO create sink
                // state.sender.send(Msg::PlayShortEffect(effect,volume)).unwrap();
            }
        }

        #[doc(hidden)]
        pub fn update_volume() {
            //TODO change due to effect volume
        }

        /// play the sound effect at the position of the listener
        /// i.e. volume is `global_volume * effect_volume`
        pub fn play_on_listener(effect: usize) {
            play(effect,super::listener());
        }

        /// stop all short sound effects
        pub fn stop_all() {
            let state = unsafe { (*RAW_STATE).read().unwrap() };
            //TODO drop short sink
            // state.sender.send(Msg::StopAllShortEffects).unwrap();
        }
    }

    pub mod persistent {
        //! this module allow to play persistent sound effects
        //!
        //! ```lua
        //! volume = global_volume * effect_volume * sum(distance(position,listener_position))
        //! ```
        //!
        //! but once a sound effect is played at a volume it doesn't change its volume anymore
        //!
        //! this can lead to weird effects for not so short sound effects and with moving source
        //!
        //! also if its volume is zero then the sound is not played at all

        use super::super::RAW_STATE;

        /// add a new source of the effect
        pub fn add_position(effect: usize, pos: [f32;3]) {
            let mut state = unsafe { (*RAW_STATE).write().unwrap() };
            state.persistent_effect_positions[effect].push(pos);
        }

        /// add a vec of new sources of the effect
        pub fn add_positions(effect: usize, mut pos: Vec<[f32;3]>) {
            let mut state = unsafe { (*RAW_STATE).write().unwrap() };
            state.persistent_effect_positions[effect].append(&mut pos);
        }

        /// add a vec of new sources of the effects
        pub fn add_positions_for_all(all: Vec<(usize,Vec<[f32;3]>)>) {
            let mut state = unsafe { (*RAW_STATE).write().unwrap() };
            for (effect,mut pos) in all {
                state.persistent_effect_positions[effect].append(&mut pos);
            }
        }

        /// remove all sources of the effect
        pub fn clear_positions(effect: usize) {
            let mut state = unsafe { (*RAW_STATE).write().unwrap() };
            state.persistent_effect_positions[effect].clear()
        }

        /// remove all sources of all effects
        pub fn clear_positions_for_all() {
            let mut state = unsafe { (*RAW_STATE).write().unwrap() };
            for p in &mut state.persistent_effect_positions {
                p.clear()
            }
        }

        /// update the volume of effect computed from sources position and listener position at the
        /// moment of this call
        pub fn update_volume(effect: usize) {
            use std::ops::Mul;

            let state = unsafe { (*RAW_STATE).read().unwrap() };
            let v = state.persistent_effect_positions[effect].iter()
                .fold(0f32, |acc, &pos| acc + state.distance_model.distance(pos,state.listener))
                .mul(state.effect_volume)
                .mul(state.global_volume);

            //TODO change volume of sink
            //TODO take mute indo consideration
            // state.sender.send(Msg::UpdatePersistentEffectVolume(effect,v)).unwrap();
        }

        /// update the volume of all effect
        pub fn update_volume_for_all() {
            use std::ops::Mul;

            let state = unsafe { (*RAW_STATE).read().unwrap() };

            let mut volumes = Vec::with_capacity(state.persistent_effect_positions.len());

            for effect_positions in &state.persistent_effect_positions {
                volumes.push(effect_positions.iter()
                    .fold(0f32, |acc, &pos| acc + state.distance_model.distance(pos,state.listener))
                    .mul(state.effect_volume)
                    .mul(state.global_volume));
            }

            //TODO change volume of sink
            //TODO take mute indo consideration
            // state.sender.send(Msg::UpdatePersistentEffectsVolume(volumes)).unwrap();
        }

        /// pause all persistent effects
        pub fn mute_all() {
            let mut state = unsafe { (*RAW_STATE).write().unwrap() };
            if !state.persistent_mute {
                state.persistent_mute = true;
                // TODO set volume 0 for sink
                // state.sender.send(Msg::SetAllPersistentMute(true)).unwrap();
            }
        }

        /// resume all persistent effects
        pub fn unmute_all() {
            let mut state = unsafe { (*RAW_STATE).write().unwrap() };
            if state.persistent_mute {
                state.persistent_mute = false;
                // TODO set volume not 0
                // state.sender.send(Msg::SetAllPersistentMute(false)).unwrap();
            }
        }

        /// return whereas persistent effects are muted
        pub fn is_all_mute() -> bool {
            let state = unsafe { (*RAW_STATE).read().unwrap() };
            state.persistent_mute
        }
    }

    /// set the position of the listener
    pub fn set_listener(pos: [f32;3]) {
        let mut state = unsafe { (*RAW_STATE).write().unwrap() };
        state.listener = pos;
    }

    /// return the position of the listener
    pub fn listener() -> [f32;3] {
        let state = unsafe { (*RAW_STATE).read().unwrap() };
        state.listener
    }

    /// set the distance model
    pub fn set_distance_model(d: DistanceModel) {
        let mut state = unsafe { (*RAW_STATE).write().unwrap() };
        state.distance_model = d;
    }

    /// distance model, used to compute sound effects volumes.
    #[derive(Clone,Debug,PartialEq,RustcDecodable,RustcEncodable)]
    pub enum DistanceModel {
        /// if d <= a then 1
        ///
        /// if a <= d <= b then 1-((d-a)/(b-a))
        ///
        /// if d >= b then 0
        Linear(f32,f32),
        /// if d <= a then 1
        ///
        /// if a <= d <= b then (1-((d-a)/(b-a)))^2
        ///
        /// if d >= b then 0
        Pow2(f32,f32),
    }

    impl DistanceModel {
        fn distance(&self, pos: [f32;3], listener: [f32;3]) -> f32 {
            let d = pos.iter()
                .zip(&listener)
                .map(|(a,b)| (a-b).powi(2))
                .fold(0.,|sum,i| sum+i)
                .sqrt();

            match *self {
                DistanceModel::Linear(a,b) => {
                    if d <= a {
                        1.
                    } else if d <= b {
                        1. - ((d-a)/(b-a))
                    } else {
                        0.
                    }
                }
                DistanceModel::Pow2(a,b) => {
                    if d <= a {
                        1.
                    } else if d <= b {
                        (1. - ((d-a)/(b-a))).powi(2)
                    } else {
                        0.
                    }
                }
            }
        }
    }

    #[test]
    fn test_distance() {
        let origin = [0.,0.,0.];
        let d = DistanceModel::Linear(10.,110.);
        assert_eq!(d.distance(origin,origin), 1.);
        assert_eq!(d.distance(origin,[10.,0.,0.]), 1.);
        assert_eq!(d.distance(origin,[60.,0.,0.]), 0.5);
        assert!(d.distance(origin,[100.,0.,0.]) - 0.1 < 0.00001);
        assert_eq!(d.distance(origin,[150.,0.,0.]), 0.);
    }

}

pub mod music {
    //! this module allow to play music

    use super::RAW_STATE;

    /// set the volume of the music
    /// the actual music volume is `music_volume * global_volume`
    pub fn set_volume(v: f32) {
        let mut state = unsafe { (*RAW_STATE).write().unwrap() };
        state.music_volume = v;
        //TODO set sink volume
        // state.sender.send(Msg::SetMusicVolume(state.music_volume*state.global_volume)).unwrap();
    }

    /// return the volume of the music
    pub fn volume() -> f32 {
        let state = unsafe { (*RAW_STATE).read().unwrap() };
        state.music_volume
    }

    /// play the music
    //TODO rename set_music
    pub fn play(music: usize) {
        let mut state = unsafe { (*RAW_STATE).write().unwrap() };

        state.music_index = Some(music);
        //TODO create a new sink
        // let snd_file = SndFile::new(&state.music[music],OpenMode::Read).unwrap();
        // state.sender.send(Msg::PlayMusic(snd_file)).unwrap();
    }

    /// play the music if is different from the current one
    pub fn play_or_continue(music: usize) {
        let must_play = if let Some(index) = index() {
            music != index
        } else {
            true
        };

        if must_play {
            play(music);
        }
    }

    /// pause the music
    //TODO rename mute
    pub fn pause() {
        let state = unsafe { (*RAW_STATE).read().unwrap() };
        //TODO set volume
        // state.sender.send(Msg::PauseMusic).unwrap();
    }

    /// resume the music
    //TODO rename unmute
    pub fn resume() {
        let state = unsafe { (*RAW_STATE).read().unwrap() };
        //TODO set volume
        // state.sender.send(Msg::ResumeMusic).unwrap();
    }

    /// stop the music
    pub fn stop() {
        let mut state = unsafe { (*RAW_STATE).write().unwrap() };
        state.music_index = None;
        //TODO drop sink
        // state.sender.send(Msg::StopMusic).unwrap();
    }

    /// return the current type of transition
    pub fn transition() -> MusicTransition {
        let state = unsafe { (*RAW_STATE).read().unwrap() };
        state.music_transition
    }

    /// set the type of transition between musics
    pub fn set_transition(trans: MusicTransition) {
        let mut state = unsafe { (*RAW_STATE).write().unwrap() };
        state.music_transition = trans;
    }

    /// return the index of the current music if any
    pub fn index() -> Option<usize> {
        let state = unsafe { (*RAW_STATE).read().unwrap() };
        state.music_index
    }

    /// the type of transition between musics
    #[derive(Clone,Copy,Debug,PartialEq,RustcDecodable,RustcEncodable)]
    pub enum MusicTransition {
        /// the current music end smoothly and then the new one is played. (in second)
        Smooth(f32),
        /// the current music end smoothly while the new one begin smoothly. (in second)
        Overlap(f32),
        /// the current music is stopped and the new one is played.
        Instant,
    }

    impl MusicTransition {
        /// whether music transition is smooth
        pub fn is_smooth(&self) -> bool {
            if let &MusicTransition::Smooth(_) = self {
                true
            } else {
                false
            }
        }
    }
}

/// set the global volume
pub fn set_volume(v: f32) {
    let mut state = unsafe { (*RAW_STATE).write().unwrap() };
    state.global_volume = v;
    // TODO change sink volume
    // state.sender.send(Msg::SetMusicVolume(state.music_volume*state.global_volume)).unwrap();
}

/// return the global volume
pub fn volume() -> f32 {
    let state = unsafe { (*RAW_STATE).read().unwrap() };
    state.global_volume
}

/// stop music and effects
pub fn stop() {
    let state = unsafe { (*RAW_STATE).read().unwrap() };
    music::stop();
    effect::short::stop_all();
    effect::persistent::mute_all();
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
