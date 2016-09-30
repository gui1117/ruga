extern crate baal;

use std::thread;
use std::time::Duration;

#[test]
fn test() {
    let setting = baal::Setting {
        channels: 1,
        sample_rate: 44100.0,
        frames_per_buffer: 64,

        effect_dir: "assets/effects".into(),
        music_dir: "assets/musics".into(),

        global_volume: 0.0,
        music_volume: 0.5,
        effect_volume: 0.5,

        distance_model: baal::effect::DistanceModel::Linear(10.,110.),

        music_loop: true,

        music_transition: baal::music::MusicTransition::Instant,

        short_effect: vec!("shoot.ogg".into(),"hit.ogg".into()),
        persistent_effect: vec!(),
        music: vec!("village.ogg".into()),

        check_level: baal::CheckLevel::Always,
    };

    for _ in 0..4 {
        baal::init(&setting).expect("fail to init baal");
        baal::music::play(0);

        for i in 0..7 {
            let p = (i*20) as f32;
            baal::effect::short::play(0,[p,0.,0.]);
            thread::sleep(Duration::from_millis(1));
            baal::effect::short::play(1,[p,0.,0.]);
            thread::sleep(Duration::from_millis(1));
        }

        baal::close();
    }
}
