use toml;

use graphics;
use std;

const CONFIG_FILE: &'static str = "config.toml";

fn load_configuration() -> Result<Configuration, Error> {
    use std::fs::File;
    use std::io::Read;
    let mut config = String::new();
    File::open(CONFIG_FILE).unwrap().read_to_string(&mut config).unwrap();
    Ok(toml::from_str(&config)?)
}

lazy_static! {
    pub static ref CONFIG: Configuration = load_configuration().unwrap();
}

#[derive(Deserialize)]
pub struct Configuration {
    pub number_of_thread: usize, // TODO resolve it at runtime
    pub persistent_snd_cooldown: usize,
    pub difficulty: f32,
    pub keys: Keys,
    pub effect: Effect,
    pub physic: Physic,
    pub touch: Touch,
    pub joystick: Joystick,
    pub menu: Menu,
    pub entities: Entities,
    pub levels: Levels,
    pub audio: Audio,
    pub window: Window,
    pub graphics: Graphics,
    pub text: Text,
    pub camera: Camera,
    pub event_loop: EventLoop,
}

#[derive(Deserialize)]
pub struct Keys {
    pub up: Vec<u8>,
    pub down: Vec<u8>,
    pub left: Vec<u8>,
    pub right: Vec<u8>,
    pub escape: Vec<u8>,
}

#[derive(Deserialize)]
pub struct Effect {
    pub color: graphics::Color,
    pub angles: Vec<f32>,
    pub persistance: f32,
    pub thickness: f32,
    pub inner_length: f32,
    pub length: f32,
}

#[derive(Deserialize)]
pub struct Physic {
    pub rate: f32,
    pub unit: f32,
}

#[derive(Deserialize)]
pub struct Touch {
    pub joystick_rec: [f64; 4],
    pub joystick_radius: f64,
    pub escape_rec: [f64; 4],
}

#[derive(Deserialize)]
pub struct Joystick {
    pub time_to_repeat: f32,
    pub time_to_start_repeating: f32,
    pub press_epsilon: f32,
    pub release_epsilon: f32,
}

#[derive(Deserialize)]
pub struct Menu {
    pub entry_color: graphics::Color,
    pub cursor_color: graphics::Color,
    pub background_color: graphics::Color,
    pub clic_snd: usize,

    pub background_width: f32,
    pub background_height: f32,
}

#[derive(Deserialize)]
pub struct Entities {
    pub text_color: graphics::Color,

    pub ball_group: u32,
    pub ball_mask: u32,
    pub ball_killer_mask: u32,
    pub ball_kill_snd: usize,
    pub ball_die_snd: usize,
    pub ball_radius: f32,
    pub ball_velocity: f32,
    pub ball_time: f32,
    pub ball_weight: f32,
    pub ball_color: graphics::Color,
    pub ball_layer: graphics::Layer,
    pub ball_vel_snd_coef: f32,
    pub ball_vel_snd: usize,

    pub laser_group: u32,
    pub laser_mask: u32,
    pub laser_killer_mask: u32,
    pub laser_kill_snd: usize,
    pub laser_radius: f32,
    pub laser_color: graphics::Color,
    pub laser_layer: graphics::Layer,
    pub laser_persistent_snd: usize,

    pub column_group: u32,
    pub column_mask: u32,
    pub column_radius: f32,
    pub column_color: graphics::Color,
    pub column_layer: graphics::Layer,
    pub column_cooldown: f32,
    pub column_spawn_snd: usize,

    pub char_group: u32,
    pub char_mask: u32,
    pub char_radius: f32,
    pub char_velocity: f32,
    pub char_time: f32,
    pub char_weight: f32,
    pub char_color: graphics::Color,
    pub char_layer: graphics::Layer,
    pub char_die_snd: usize,
    pub char_restart: f32,

    pub wall_group: u32,
    pub wall_mask: u32,
    pub wall_radius: f32,
    pub wall_color: graphics::Color,
    pub wall_layer: graphics::Layer,

    pub monster_vision_mask: u32,
    pub monster_killer_mask: u32,
    pub monster_kill_snd: usize,
    pub monster_die_snd: usize,
    pub monster_group: u32,
    pub monster_mask: u32,
    pub monster_vision_time: f32,
    pub monster_radius: f32,
    pub monster_velocity: f32,
    pub monster_time: f32,
    pub monster_weight: f32,
    pub monster_color: graphics::Color,
    pub monster_layer: graphics::Layer,
    pub monster_persistent_snd: usize,

    pub portal_end_color: graphics::Color,
    pub portal_end_layer: graphics::Layer,
    pub portal_start_color: graphics::Color,
    pub portal_start_layer: graphics::Layer,
    pub portal_snd: usize,
}

#[derive(Deserialize)]
pub struct Levels {
    pub hall_length: usize,
    pub corridor_length: usize,
    pub dir: String,
    pub entry_music: String,
    pub check_level: bool,

    pub empty_col: [u8; 3],
    pub char_col: [u8; 3],
    pub portal_col: [u8; 3],
    pub laser_col: [u8; 3],
    pub monster_col: [u8; 3],
    pub column_col: [u8; 3],
    pub wall_col: [u8; 3],
}

#[derive(Deserialize)]
pub struct Audio {
    // pub effect_dir: Vec<String>,
    // pub music_dir: Vec<String>,
    // pub global_volume: f32 save global_volume,
    // pub music_volume: f32 save music_volume,
    // pub effect_volume: f32 save effect_volume,
    // pub distance_model: Die String [linear, pow2],
    // pub distance_model_min: f32,
    // pub distance_model_max: f32,
    // pub short_effects: VecVecStringPath,
    // pub persistent_effects: VecVecStringPath,
    // pub transition_type: e String [instant, smooth, overlap],
    // pub transition_time: u64,
}

#[derive(Deserialize)]
pub struct Window {
    pub dimension: [u32; 2],
    pub vsync: bool,
    pub multisampling: u16,
    pub fullscreen: bool,
    pub fullscreen_on_primary_monitor: bool,
    pub fullscreen_monitor: usize,
}

#[derive(Deserialize)]
pub struct Graphics {
    pub base03: [f32; 4],
    pub base02: [f32; 4],
    pub base01: [f32; 4],
    pub base00: [f32; 4],
    pub base0: [f32; 4],
    pub base1: [f32; 4],
    pub base2: [f32; 4],
    pub base3: [f32; 4],
    pub yellow: [f32; 4],
    pub orange: [f32; 4],
    pub red: [f32; 4],
    pub magenta: [f32; 4],
    pub violet: [f32; 4],
    pub blue: [f32; 4],
    pub cyan: [f32; 4],
    pub green: [f32; 4],
    pub mode: graphics::Mode,
    pub luminosity: f32,
    pub circle_precision: usize,
    pub font_file: String,
    pub font_size: u32,
    pub font_scale: f32,
}

#[derive(Deserialize)]
pub struct Text {
    pub top: i32,
    pub bottom: i32,
    pub right: i32,
    pub left: i32,
}

#[derive(Deserialize)]
pub struct Camera {
    pub zoom: f32,
}

#[derive(Deserialize)]
pub struct EventLoop {
    pub ups: u64,
    pub max_fps: u64,
}

#[derive(Debug)]
enum Error {
    Io(::std::io::Error),
    Toml(toml::de::Error),
}
impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::Io(err)
    }
}
impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::Toml(err)
    }
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        use self::Error::*;
        match *self {
            Io(ref e) => write!(fmt, "file `{}`: io error: {}", CONFIG_FILE, e),
            Toml(ref e) => write!(fmt, "file `{}`: toml decode error: {}", CONFIG_FILE, e),
        }
    }
}
