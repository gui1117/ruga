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
