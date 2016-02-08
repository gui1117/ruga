use std::thread;
use std::collections::HashMap;
use std::path::Path;
use sndfile::{
    SndFile,
    OpenMode,
};
use portaudio::PortAudio;
use portaudio as pa;

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 48_000.0;
const FRAMES_PER_BUFFER: u32 = 64;
//const TABLE_SIZE: usize = 200;

pub mod sounds {
    pub const PISTOL: u32 = 0;
    pub const BOMB: u32 = 1;
    pub const CLICK: u32 = 2;
}

pub mod musics {
    pub const ChampionOfLight: u32 = 0;
    pub const IceFlow: u32 = 1;
    pub const InAHeartbeat: u32 = 2;
    pub const LatinIndustries: u32 = 3;
}

pub struct SoundManager {
    listener: [f64;2],
}

impl SoundManager {
    pub fn new(x: f64, y: f64) -> SoundManager {
        thread::spawn(move || {
            let sound_path = Path::new("assets/musics/Champion_of_Light.ogg");
            let mut sound_file = SndFile::new(sound_path,OpenMode::Read).unwrap();

            //let mut left_phase = 0;
            //let mut right_phase = 0;

            let pa = pa::PortAudio::new().unwrap();

            let mut settings = pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER).unwrap();
            // we won't output out of range samples so don't bother clipping them.
            //settings.flags = pa::stream_flags::CLIP_OFF;

            // This routine will be called by the PortAudio engine when audio is needed. It may called at
            // interrupt level on some machines so don't do anything that could mess up the system like
            // dynamic resource allocation or IO.
            let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
                sound_file.readf_f32(buffer,frames as i64);
                for f in buffer {
                    *f /= 10.;
                }
                pa::Continue
            };

            let mut stream = pa.open_non_blocking_stream(settings, callback).unwrap();

            stream.start().unwrap();
            pa.sleep(NUM_SECONDS * 1_000_000);
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











