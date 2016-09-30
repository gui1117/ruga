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

    baal::init(&setting).expect("fail to init baal");

    let child = std::thread::spawn(|| {
        for _ in 0..20 {
            baal::effect::short::play(0,[0.,0.,0.]);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for _ in 0..20 {
        baal::reset(&setting).expect("fail to reset baal");
        thread::sleep(Duration::from_millis(1));
    }
    child.join().unwrap();
    baal::close();
}
