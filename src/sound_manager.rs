use std::thread;
use std::path::Path;
use sndfile::{
    SndFile,
    OpenMode,
    SeekMode,
};
use portaudio::PortAudio;
use portaudio as pa;
use std::i32;
use std::sync::mpsc::{
    channel,
    Sender,
};
use std::ops::Rem;

const CHANNELS: i32 = 2;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const BUFFER_SIZE: usize = (CHANNELS as usize) * (FRAMES_PER_BUFFER as usize);

pub mod sounds {
    pub const RIFLE: u32 =      0;
    pub const SNIPER: u32 =     1;
    pub const SHOTGUN: u32 =    2;
    pub const SWORD: u32 =      3;
}

struct Sound {
    start: usize,
    end: usize,
    batch: Vec<SndFile>,
    volume: Vec<f32>,
}

impl Sound {
    fn new(name: &str, capacity: usize) -> Sound {
        let s = format!("assets/sounds/{}",name);
        let path = Path::new(&s);

        let mut batch = Vec::with_capacity(capacity);
        let mut volume = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            batch.push(SndFile::new(path,OpenMode::Read).unwrap());
            volume.push(0.5);
        }

        Sound {
            start: 0,
            end: 0,
            batch: batch,
            volume: volume,
        }
    }

    fn fill_buffers(&mut self, buffers: &mut Vec<[f32; BUFFER_SIZE]>, index: &mut usize, frames: i64) {
        let len = if self.start <= self.end {
            self.end-self.start
        } else {
            (self.end+self.batch.len())-self.start
        };
        if buffers.len() - *index < len {
            for _ in 0..*index + len - buffers.len() {
                buffers.push([0.;BUFFER_SIZE]);
            }
        }

        let range = if self.start <= self.end {
            (self.start..self.end).chain(0..0)
        } else {
            (0..self.end).chain(self.start..self.batch.len())
        };

        let mut counter = *index;
        for i in range {
            let frame = self.batch[i].readf_f32(&mut buffers[counter],frames);
            for k in 0..BUFFER_SIZE {
                buffers[counter][k] *= self.volume[i];
            }
            if frame == 0 {
                self.start = (self.start+1).rem(self.batch.len());
            }
            counter += 1;
        }
        *index = counter;
    }

    fn play(&mut self,volume: f32) {
        self.volume[self.end] = volume;
        self.batch[self.end].seek(0,SeekMode::SeekSet);

        self.end = (self.end+1).rem(self.batch.len());
        if self.start == self.end {
            self.start = (self.start+1).rem(self.batch.len());
        }
    }
}

struct Music {
    snd_file: SndFile,
    volume: f32,
}

impl Music {
    fn new(name: &str) -> Music {
        let s = format!("assets/musics/{}",name);
        let path = Path::new(&s);

        Music {
            snd_file: SndFile::new(path,OpenMode::Read).unwrap(),
            volume: 0.5,
        }
    }

    fn fill_buffers(&mut self, buffers: &mut Vec<[f32; BUFFER_SIZE]>, index: &mut usize, frames: i64) {
        if buffers.len() == *index {
            buffers.push([0.;BUFFER_SIZE]);
        }

        let frame = self.snd_file.readf_f32(&mut buffers[*index],frames);
        for k in 0..BUFFER_SIZE {
            buffers[*index][k] *= self.volume;
        }
        if frame == 0 {
            self.snd_file.seek(0,SeekMode::SeekSet);
        }
        *index += 1;
    }

    fn set_volume(&mut self, v: f32) {
        self.volume = v;
    }
}

enum SoundMsg {
    Play(usize,f32),
    SetMusicVolume(f32),
}

pub struct SoundManager {
    listener: [f64;2],
    music_volume: f32,
    global_volume: f32,
    sounds_volume: f32,
    start_decrease: f64,
    end_decrease: f64,
    pa_tx: Sender<SoundMsg>,
}

impl SoundManager {
    pub fn new() -> SoundManager {
        let (pa_tx,pa_rx) = channel();

        thread::spawn(move || {
            let mut music = Music::new("cylindric.ogg");
            let mut sounds = vec![
                Sound::new("rifle.ogg",40),
                Sound::new("sniper.ogg",20),
                Sound::new("shotgun.ogg",20),
                Sound::new("shotgun.ogg",20)
            ];

            let mut buffers: Vec<[f32;(BUFFER_SIZE) as usize]> = Vec::with_capacity(10);
            for _ in 0..10 {
                buffers.push([0.;(BUFFER_SIZE) as usize]);
            }

            let pa = pa::PortAudio::new().unwrap();

            let settings = pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER).unwrap();

            let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {

                while let Ok(sound_msg) = pa_rx.try_recv() {
                    match sound_msg {
                        SoundMsg::Play(n,vol) => sounds[n].play(vol),
                        SoundMsg::SetMusicVolume(vol) => music.set_volume(vol),
                    }
                }

                let mut index = 0;

                music.fill_buffers(&mut buffers,&mut index,frames as i64);
                for sound in &mut sounds {
                    sound.fill_buffers(&mut buffers,&mut index,frames as i64);
                }

                let mut i = 0;
                for f in buffer {
                    *f = buffers[0][i];
                    for k in 1..index {
                        *f += buffers[k][i]
                    }
                    i += 1;
                }
                pa::Continue
            };

            let mut stream = pa.open_non_blocking_stream(settings, callback).unwrap();

            stream.start().unwrap();

            loop {
                pa.sleep(i32::max_value());
            }
        });


        SoundManager {
            start_decrease: 1.,
            end_decrease: 100.,
            listener: [0.,0.],
            music_volume: 1.,
            global_volume: 0.5,
            sounds_volume: 1.,
            pa_tx: pa_tx,
        }
    }
    
    pub fn set_decrease_bounds(&mut self, start: f64, end: f64) {
        self.start_decrease = start;
        self.end_decrease = end;
    }

    pub fn set_listener(&mut self, pos: [f64;2]) {
        self.listener = pos;
    }

    #[allow(unused_must_use)]
    pub fn set_music_volume(&mut self, v: f32) {
        self.music_volume = v;
        self.pa_tx.send(SoundMsg::SetMusicVolume(self.music_volume*self.global_volume));
    }

    pub fn set_sounds_volume(&mut self, v: f32) {
        self.sounds_volume = v;
    }

    #[allow(unused_must_use)]
    pub fn set_global_volume(&mut self, v: f32) {
        self.global_volume = v;
        self.pa_tx.send(SoundMsg::SetMusicVolume(self.music_volume*self.global_volume));
    }

    #[allow(unused_must_use)]
    pub fn play(&mut self, x: f64, y: f64, sound: u32) {
        // the volume of sounds effects are set only at the beginning

        let dist = ((self.listener[0] - x).powi(2) + (self.listener[1] - y).powi(2)).sqrt();
        let k_dist = if dist >= self.end_decrease {
            return;
        } else if dist > self.start_decrease {
            ((self.end_decrease - dist)/(self.end_decrease - self.start_decrease)) as f32
        } else {
            1.
        };

        let vol = k_dist * self.global_volume * self.sounds_volume;
        self.pa_tx.send(SoundMsg::Play(sound as usize,vol));
    }
}

