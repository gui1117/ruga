use configuration::BitflagU32;
use levels as levelss;

pub type EffectsName = Vec<String>;
pub type EffectsNumber = Vec<u32>;
pub type Musics = Vec<String>;
pub type Dimension = [u32;2];
pub type Array4F32 = [f32;4];
pub type VecU8 = Vec<u8>;
pub type Dungeons = Vec<levelss::Dungeon>;
pub type Array3U8 = [u8;3];

fn config_constraint(conf: &Config) -> Result<(),String> {
    if conf.keys.up.len() == 0
       || conf.keys.down.len() == 0
       || conf.keys.left.len() == 0
       || conf.keys.right.len() == 0 {
           return Err("ERROR: configuration file invalid: keys mustn't be empty".into());
    }

    if conf.audio.effects_name.len() != conf.audio.effects_number.len() {
        return Err("ERROR: configuration file invalid: audio effects name and audio effects number arrays must be of the same length".into());
    }

    //TODO audio constraint

    Ok(())
}

configure!(
    file = "config.toml";
    debug_file = "config.toml";

    constraint = config_constraint;

    general: {
        number_of_thread: t usize,
    },
    keys: {
        up: t VecU8,
        down: t VecU8,
        left: t VecU8,
        right: t VecU8,
        escape: t VecU8,
    },
    physic: {
        rate: t f32,
        unit: t f32,
    },
    entities: {
        ball_group: t BitflagU32,
        ball_mask: t BitflagU32,
        ball_killer_mask: t BitflagU32,
        ball_kill_snd: t usize,
        ball_create_snd: t usize,
        ball_die_snd: t usize,
        ball_radius: t f32,
        ball_velocity: t f32,
        ball_time: t f32,
        ball_weight: t f32,
        ball_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],
        ball_layer: e String [floor,middle,ceil],

        laser_group: t BitflagU32,
        laser_mask: t BitflagU32,
        laser_killer_mask: t BitflagU32,
        laser_kill_snd: t usize,
        laser_radius: t f32,
        laser_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],
        laser_layer: e String [floor,middle,ceil],

        column_group: t BitflagU32,
        column_mask: t BitflagU32,
        column_radius: t f32,
        column_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],
        column_layer: e String [floor,middle,ceil],

        char_group: t BitflagU32,
        char_mask: t BitflagU32,
        char_radius: t f32,
        char_velocity: t f32,
        char_time: t f32,
        char_weight: t f32,
        char_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],
        char_layer: e String [floor,middle,ceil],
        char_die_snd: t usize,
        //TODO char_restart_snd: t usize,

        wall_group: t BitflagU32,
        wall_mask: t BitflagU32,
        wall_radius: t f32,
        wall_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],
        wall_layer: e String [floor,middle,ceil],

        monster_vision_mask: t BitflagU32,
        monster_killer_mask: t BitflagU32,
        monster_kill_snd: t usize,
        monster_die_snd: t usize,
        monster_group: t BitflagU32,
        monster_mask: t BitflagU32,
        monster_vision_time: t f32,
        monster_radius: t f32,
        monster_velocity: t f32,
        monster_time: t f32,
        monster_weight: t f32,
        monster_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],
        monster_layer: e String [floor,middle,ceil],

        portal_end_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],
        portal_end_layer: e String [floor,middle,ceil],
        portal_start_color: e String [base5,base4,base3,base2,base1,yellow,orange,red,magenta,violet,blue,cyan,green],
        portal_start_layer: e String [floor,middle,ceil],
        portal_snd: t usize,
    },
    levels: {
        dir: t String,
        entry_music: t usize,
        dungeons: t Dungeons,

        empty_col: t Array3U8,
        char_col: t Array3U8,
        portal_col: t Array3U8,
        laser_col: t Array3U8,
        monster_col: t Array3U8,
        column_col: t Array3U8,
        wall_col: t Array3U8,
    },
    audio: {
        channels: t i32,
        sample_rate: t f64,
        frames_per_buffer: t u32,
        effect_dir: t String,
        music_dir: t String,
        global_volume: t f32,
        music_volume: t f32,
        effect_volume: t f32,
        distance_model: e String [linear,pow2],
        distance_model_min: t f64,
        distance_model_max: t f64,
        music_loop: t bool,
        effects_name: t EffectsName,
        effects_number: t EffectsNumber,
        musics: t Musics,
        check_level: e String [always,debug,never],
        transition_type: e String [instant,smooth,overlap],
        transition_time: t f32,
    },
    window: {
        dimension: t Dimension,
        vsync: t bool,
        multisampling: t u16,
        fullscreen: t bool,
        fullscreen_on_primary_monitor: t bool,
        fullscreen_monitor: t usize,
    },
    graphics: {
        base03: t Array4F32,
        base02: t Array4F32,
        base01: t Array4F32,
        base00: t Array4F32,
        base0: t Array4F32,
        base1: t Array4F32,
        base2: t Array4F32,
        base3: t Array4F32,
        yellow: t Array4F32,
        orange: t Array4F32,
        red: t Array4F32,
        magenta: t Array4F32,
        violet: t Array4F32,
        blue: t Array4F32,
        cyan: t Array4F32,
        green: t Array4F32,
        mode: e String [light,dark],
        luminosity: t f32,
        circle_precision: t usize,
        font_precision: t u32,
        font_file: t String,
        font_ratio: t f32,
    },
    camera: {
        zoom: t f32,
    },
    event_loop: {
        ups: t u64,
        max_fps: t u64,
    },
);

