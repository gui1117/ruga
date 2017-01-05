pub mod components;
pub mod update_systems;
pub mod draw_systems;

#[derive(Clone)]
pub enum State {
    Reload(f32),
    Ready,
    Setup(f32),
    Setdown(f32),
}

#[derive(Clone)]
pub enum Kind {
    Sniper,
    Shotgun,
    // Sword,
    Hammer(bool), // a component that inform if right or left if present or not
    Uzis,
}

impl Kind {
    pub fn from_str(s: &str) -> Option<Kind> {
        match s {
            "sniper" => Some(Kind::Sniper),
            "shotgun" => Some(Kind::Shotgun),
            "hammer" => Some(Kind::Hammer(false)),
            "uzis" => Some(Kind::Uzis),
            _ => None,
        }
    }
}
