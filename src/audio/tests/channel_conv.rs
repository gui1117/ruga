extern crate baal;

use std::thread;
use std::time::Duration;

#[test]
fn channel_conv() {
    let one_channel_setting = baal::Setting {
        channels: 2,
        sample_rate: 44100.,
        frames_per_buffer: 64,

        effect_dir: "assets/effects".into(),
        music_dir: "assets/musics".into(),

        global_volume: 0.5,
        music_volume: 0.5,
        effect_volume: 0.5,

        distance_model: baal::effect::DistanceModel::Linear(10.,110.),

        music_loop: true,

        music_transition: baal::music::MusicTransition::Instant,

        short_effect: vec!("explosion.ogg".into(),"stereo_explosion.ogg".into()),
        persistent_effect: vec!(),
        music: vec!(),

        check_level: baal::CheckLevel::Always,
    };
    let two_channel_setting = baal::Setting {
        channels: 1,
        sample_rate: 44100.,
        frames_per_buffer: 64,

        effect_dir: "assets/effects".into(),
        music_dir: "assets/musics".into(),

        global_volume: 0.5,
        music_volume: 0.5,
        effect_volume: 0.5,

        distance_model: baal::effect::DistanceModel::Linear(10.,110.),

        music_loop: true,

        music_transition: baal::music::MusicTransition::Instant,

        short_effect: vec!("explosion.ogg".into(),"stereo_explosion.ogg".into()),
        persistent_effect: vec!(),
        music: vec!(),

        check_level: baal::CheckLevel::Always,
    };

    baal::init(&one_channel_setting).expect("init baal");

    baal::effect::short::play(0,[0.,0.,0.]);
    thread::sleep(Duration::from_secs(2));
    baal::effect::short::play(1,[0.,0.,0.]);
    thread::sleep(Duration::from_secs(5));

    baal::reset(&two_channel_setting).expect("reset baal");

    baal::effect::short::play(0,[0.,0.,0.]);
    thread::sleep(Duration::from_secs(2));
    baal::effect::short::play(1,[0.,0.,0.]);
    thread::sleep(Duration::from_secs(5));

    baal::close();
}
