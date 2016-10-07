//! this module allow to play music

use super::RAW_STATE;

/// set the volume of the music
/// the actual music volume is `music_volume * global_volume`
pub fn set_volume(v: f32) {
    {
        let mut state = unsafe { (*RAW_STATE).write().unwrap() };
        state.music_volume = v;
    }
    update_volume();
}

/// return the volume of the music
pub fn volume() -> f32 {
    let state = unsafe { (*RAW_STATE).read().unwrap() };
    state.music_volume
}

#[doc(hidden)]
pub fn update_volume() {
    let mut state = unsafe { (*RAW_STATE).write().unwrap() };
    state.music_sink.set_volume(state.music_volume * state.global_volume);
}

/// play the music
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
