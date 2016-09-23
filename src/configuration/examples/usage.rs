#[macro_use] extern crate configuration;
#[macro_use] extern crate lazy_static;
extern crate toml;

pub type array3 = [u8;3];

fn config_constraint(conf: &Config) -> Result<(),String> {
    Ok(())
}

configure!(
    file = "config.toml";
    debug_file = "config.toml";
    save_file= "save.toml";

    constraint = config_constraint;

    general: {
        view: t f32,
        fullscreen: t usize save fullscreen,
        safety: e String [weak,robust] save safety,
    },
    control: {
        up: t array3 save up,
        down: t u8,
    },
);

fn main() {
    assert!(config.general.view == 45f32);
    assert!(config.general.fullscreen == 4usize);
    assert!(config.control.up == [1u8,2u8,3u8]);
    assert!(config.control.down == 5u8);
    assert!(config.general.safety == String::from("weak"));

    assert!(save(Save {
        fullscreen: 4,
        safety: "weak".into(),
        up: [1,2,3],
    }).is_ok());
}
