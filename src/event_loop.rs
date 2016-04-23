use glium::backend::glutin_backend::GlutinFacade;
use glium::Frame;
use glium::glutin;

use std::thread::sleep;
use std::time::Duration;
use time;
use std::cmp;
use yaml_rust::yaml;

pub struct RenderArgs {
    pub ext_dt: f64,
    pub frame: Frame
}

pub struct IdleArgs {
    pub dt: f64,
}

pub struct UpdateArgs {
    pub dt: f64,
}

pub enum Event {
    Render(RenderArgs),
    Update(UpdateArgs),
    Idle(IdleArgs),
    Input(glutin::Event),
}

#[derive(Clone,Debug)]
pub struct Setting {
    ups: u64,
    max_fps: u64,
}

impl Setting {
    pub fn from_yaml(config: &yaml::Yaml) -> Result<Setting, String> {
        let hash = try!(config.as_hash().ok_or("event loop setting not associative array"));
        let ups = try!(try!(hash.get(&yaml::Yaml::from_str("ups"))
                                      .ok_or("event loop setting must have ups key"))
                       .as_i64()
                       .ok_or("event loop setting ups must be an integer")) as u64;

        let max_fps = try!(try!(hash.get(&yaml::Yaml::from_str("max_fps"))
                                      .ok_or("event loop setting must have max_fps key"))
                       .as_i64()
                       .ok_or("event loop setting max_fps must be an integer")) as u64;

        Ok(Setting {
            ups: ups,
            max_fps: max_fps,
        })
    }
}

/// A trait for create event iterator from window.
pub trait Events {
    /// Creates event iterator from window.
    fn events(&self, &Setting) -> WindowEvents;
}

impl Events for GlutinFacade {
    fn events(&self, setting: &Setting) -> WindowEvents {
        WindowEvents::new(setting)
    }
}

/// Tells whether last emitted event was idle or not.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Idle {
    No,
    Yes
}


#[derive(Copy, Clone, Debug)]
enum State {
    Render,
    UpdateLoop(Idle),
    HandleEvents,
    Update,
}

/// Methods implements for event loop settings.
pub trait EventLoop: Sized {
    /// The number of updates per second
    ///
    /// This is the fixed update rate on average over time.
    /// If the event loop lags, it will try to catch up.
    fn set_ups(&mut self, frames: u64);

    /// The number of updates per second
    ///
    /// This is the fixed update rate on average over time.
    /// If the event loop lags, it will try to catch up.
    fn ups(mut self, frames: u64) -> Self {
        self.set_ups(frames);
        self
    }

    /// The maximum number of frames per second
    ///
    /// The frame rate can be lower because the
    /// next frame is always scheduled from the previous frame.
    /// This causes the frames to "slip" over time.
    fn set_max_fps(&mut self, frames: u64);

    /// The maximum number of frames per second
    ///
    /// The frame rate can be lower because the
    /// next frame is always scheduled from the previous frame.
    /// This causes the frames to "slip" over time.
    fn max_fps(mut self, frames: u64) -> Self {
        self.set_max_fps(frames);
        self
    }

    /// Enable or disable automatic swapping of buffers.
    fn set_swap_buffers(&mut self, enable: bool);

    /// Enable or disable automatic swapping of buffers.
    fn swap_buffers(mut self, enable: bool) -> Self {
        self.set_swap_buffers(enable);
        self
    }

    /// Enable or disable benchmark mode.
    /// When enabled, it will render and update without sleep and ignore input.
    /// Used to test performance by playing through as fast as possible.
    fn set_bench_mode(&mut self, enable: bool);

    /// Enable or disable benchmark mode.
    /// When enabled, it will render and update without sleep and ignore input.
    /// Used to test performance by playing through as fast as possible.
    fn bench_mode(mut self, enable: bool) -> Self {
        self.set_bench_mode(enable);
        self
    }
}

/// An event loop iterator
///
/// *Warning: Because the iterator polls events from the window back-end,
/// it must be used on the same thread as the window back-end (usually main thread),
/// unless the window back-end supports multi-thread event polling.*
#[derive(Copy, Clone)]
pub struct WindowEvents {
    state: State,
    last_update: u64,
    last_frame: u64,
    dt_update_in_ns: u64,
    dt_frame_in_ns: u64,
    dt: f64,
    swap_buffers: bool,
    bench_mode: bool,
}

static BILLION: u64 = 1_000_000_000;

fn ns_to_duration(ns: u64) -> Duration {
    let secs = ns / BILLION;
    let nanos = (ns % BILLION) as u32;
    Duration::new(secs, nanos)
}

/// The default updates per second.
pub const _DEFAULT_UPS: u64 = 120;
/// The default maximum frames per second.
pub const _DEFAULT_MAX_FPS: u64 = 60;

impl WindowEvents {
    /// Creates a new event iterator with default UPS and FPS settings.
    pub fn new(setting: &Setting) -> WindowEvents {
        let start = time::precise_time_ns();
        let updates_per_second = setting.ups;
        let max_frames_per_second = setting.max_fps;
        WindowEvents {
            state: State::Render,
            last_update: start,
            last_frame: start,
            dt_update_in_ns: BILLION / updates_per_second,
            dt_frame_in_ns: BILLION / max_frames_per_second,
            dt: 1.0 / updates_per_second as f64,
            swap_buffers: true,
            bench_mode: false,
        }
    }

    /// Returns the next game event.
    pub fn next(&mut self, window: &mut GlutinFacade) -> Option<Event> {
        loop {
            self.state = match self.state {
                State::Render => {
                    if self.bench_mode {
                        // In benchmark mode, pretend FPS is perfect.
                        self.last_frame += self.dt_frame_in_ns;
                    } else {
                        // In normal mode, let the FPS slip if late.
                        self.last_frame = time::precise_time_ns();
                    }

                    self.state = State::UpdateLoop(Idle::No);
                    return Some(Event::Render(RenderArgs {
                        // Extrapolate time forward to allow smooth motion.
                        ext_dt: (self.last_frame - self.last_update) as f64 / BILLION as f64,
                        frame: window.draw(),
                    }));
                }
                State::UpdateLoop(ref mut idle) => {
                    if self.bench_mode {
                        // In benchmark mode, pick the next event without sleep.
                        // Idle and input events are ignored.
                        let next_frame = self.last_frame + self.dt_frame_in_ns;
                        let next_update = self.last_update + self.dt_update_in_ns;
                        let next_event = cmp::min(next_frame, next_update);
                        if next_event == next_frame {
                            State::Render
                        } else {
                            State::HandleEvents
                        }
                    } else {
                        let current_time = time::precise_time_ns();
                        let next_frame = self.last_frame + self.dt_frame_in_ns;
                        let next_update = self.last_update + self.dt_update_in_ns;
                        let next_event = cmp::min(next_frame, next_update);
                        if next_event > current_time {
                            if let Some(x) = window.poll_events().next() {
                                *idle = Idle::No;
                                return Some(Event::Input(x));
                            } else if *idle == Idle::No {
                                *idle = Idle::Yes;
                                let seconds = ((next_event - current_time) as f64) / (BILLION as f64);
                                return Some(Event::Idle(IdleArgs { dt: seconds }))
                            }
                            sleep(ns_to_duration(next_event - current_time));
                            State::UpdateLoop(Idle::No)
                        } else if next_event == next_frame {
                            State::Render
                        } else {
                            State::HandleEvents
                        }
                    }
                }
                State::HandleEvents => {
                    if self.bench_mode {
                        // Ignore input to prevent it affecting the benchmark.
                        match window.poll_events().next() {
                            None => State::Update,
                            Some(_) => State::HandleEvents,
                        }
                    } else {
                        // Handle all events before updating.
                        match window.poll_events().next() {
                            None => State::Update,
                            Some(x) => { return Some(Event::Input(x)); },
                        }
                    }
                }
                State::Update => {
                    self.state = State::UpdateLoop(Idle::No);
                    self.last_update += self.dt_update_in_ns;
                    return Some(Event::Update(UpdateArgs{ dt: self.dt }));
                }
            };
        }
    }
}

impl EventLoop for WindowEvents {
    fn set_ups(&mut self, frames: u64) {
        self.dt_update_in_ns = BILLION / frames;
        self.dt = 1.0 / frames as f64;
    }

    fn set_max_fps(&mut self, frames: u64) {
        self.dt_frame_in_ns = BILLION / frames;
    }

    fn set_swap_buffers(&mut self, enable: bool) {
        self.swap_buffers = enable;
    }

    fn set_bench_mode(&mut self, enable: bool) {
        self.bench_mode = enable;
    }
}

