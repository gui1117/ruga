
#[derive(Debug,Clone,Copy)]
pub enum Color {
    Base1,
    Base2,
    Base3,
    Base4,
    Base5,
    Yellow,
    Orange,
    Red,
    Magenta,
    Violet,
    Blue,
    Cyan,
    Green,
}

#[derive(Debug,Clone,Copy)]
pub enum Mode {
    Light,
    Dark,
}

impl Color {
    pub fn color(&self, mode: Mode) -> {
        match self {
            Color::Base1 => 
                Color::Base2 => 
                Color::Base3 => 
                Color::Base4 => 
                Color::Base5 => 
                Color::Yellow =>
                Color::Orange =>
                Color::Red =>
                Color::Magenta =>
                Color::Violet =>
                Color::Blue =>
                Color::Cyan =>
                Color::Green =>
        }
    }
}
