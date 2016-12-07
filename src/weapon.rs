#[derive(Clone)]
pub enum State {
    Reload(f32),
    Ready,
    Setup(f32),
    Setdown(f32),
}

// TODO impl luaread for kind
#[derive(Clone)]
pub enum Kind {
    Sniper,
    Shotgun,
    // Sword,
    Hammer, // a component that inform if right or left if present or not
    Uzis,
}

// TODO delete this in the final edition because lua will give parameter
pub fn sniper() -> ::components::Weapon {
    ::components::Weapon {
        reload_factor: 1.,
        setup_factor: 1.,
        setdown_factor: 1.,
        state: State::Setup(0.),
        kind: Kind::Sniper,
    }
}
