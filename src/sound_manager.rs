use std::thread;
use std::collections::HashMap;
use std::path::Path;
use sndfile::{
    SndFile,
    OpenMode,
};
use portaudio::PortAudio;
use portaudio as pa;
use std::i32;
use std::sync::mpsc::channel;

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

pub mod sounds {
    pub const RIFLE: u32 = 0;
    pub const SNIPER: u32 = 1;
    pub const SHOTGUN: u32 = 2;
    //pub const BOMB: u32 = 1;
    //pub const CLICK: u32 = 2;
}

//pub mod musics {
//    pub const Cylindric: u32 = 0;
//}

pub struct SoundManager {
    listener: [f64;2],
}

impl SoundManager {
    pub fn new(x: f64, y: f64) -> SoundManager {
        //let (s_tx,s_rx) = channel();
        thread::spawn(move || {
            let mut music = SndFile::new(Path::new("assets/musics/cylindric.ogg"),OpenMode::Read).unwrap();
            let mut sound = Vec::new();
            sound.push(SndFile::new(Path::new("assets/sounds/rifle.ogg"),OpenMode::Read).unwrap());
            sound.push(SndFile::new(Path::new("assets/sounds/sniper.ogg"),OpenMode::Read).unwrap());
            sound.push(SndFile::new(Path::new("assets/sounds/shotgun.ogg"),OpenMode::Read).unwrap());

            let pa = pa::PortAudio::new().unwrap();

            let settings = pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER).unwrap();

            let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
                music.readf_f32(buffer,frames as i64);
                for f in buffer {
                    *f /= 10.;
                }
                pa::Continue
            };

            let mut stream = pa.open_non_blocking_stream(settings, callback).unwrap();

            stream.start().unwrap();
            pa.sleep(i32::max_value());
        });


        SoundManager {
            listener: [x,y],
        }
    }

    pub fn add_sound(&mut self, x: f64, y: f64, sound: u32) {
        // the volume of sounds effects are set only at the beginning
        // send the sound number with the volume desired
    }
}











