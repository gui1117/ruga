extern crate baal;

use std::thread;
use std::time::Duration;

fn main() {
    let setting = baal::Setting {
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
        persistent_effect: vec!("electro_fly_from_xonotic_game.ogg".into()),
        music: vec!("village.ogg".into()),

        check_level: baal::CheckLevel::Always,
    };

    baal::init(&setting).unwrap();
    baal::music::play(0);

    baal::effect::set_listener([1.,1.,1.]);

    baal::effect::persistent::add_position(0,[0.0,0.0,0.0]);
    baal::effect::persistent::add_position(0,[0.0,0.0,10.0]);
    baal::effect::persistent::update_volume_for_all();

    baal::effect::short::play(0,[0.,0.,0.]);

    thread::sleep(Duration::from_secs(40));

    baal::close();
}
