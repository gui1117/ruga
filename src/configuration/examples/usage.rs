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

    constraint = config_constraint;

    general: {
        view: t f32,
        fullscreen: t usize,
        safety: e String [weak,robust],
    },
    control: {
        up: t array3,
        down: t u8,
    },
);

fn main() {
    assert!(config.general.view == 45f32);
    assert!(config.general.fullscreen == 1usize);
    assert!(config.control.up == [4u8,5u8,6u8]);
    assert!(config.control.down == 5u8);
    assert!(config.general.safety == String::from("robust"));
}
