use frame_manager::{color, FrameManager};
use sound_manager::{SoundManager, sounds};

pub struct Position {
    x: f64,
    y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position {
            x: x,
            y: y,
        }
    }
}

pub struct Line {
    x: f64,
    y: f64,
    angle: f64,
    length: f64
}

impl Line {
    pub fn new(x: f64, y: f64, angle: f64, length: f64) -> Line {
        Line {
            x: x,
            y: y,
            angle: angle,
            length: length,
        }
    }
}

pub enum Effect {
    SwordAttack(Line),
    SniperShoot(Line),
    ShotgunShoot(Vec<Line>),
    RifleShoot(Line),
    WallDecision(Position),
}

pub struct EffectManager {
    effects: Vec<Effect>,
}

impl EffectManager {
    pub fn new() -> EffectManager {
        EffectManager {
            effects: Vec::new(),
        }
    }

    pub fn add(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn render(&mut self, frame_manager: &mut FrameManager, sound_manager: &mut SoundManager) {
        while let Some(effect) = self.effects.pop() {
            match effect {
                Effect::SwordAttack(line) => render_sword_attack(&line,frame_manager,sound_manager),
                Effect::SniperShoot(line) => render_sniper_shoot(&line,frame_manager,sound_manager),
                Effect::ShotgunShoot(lines) => render_shotgun_shoot(&lines,frame_manager,sound_manager),
                Effect::RifleShoot(line) => render_rifle_shoot(&line,frame_manager,sound_manager),
                Effect::WallDecision(position) => sound_manager.play(position.x,position.y,sounds::MOVING_WALL),
            }
        }
    }

}

fn render_sword_attack(line: &Line, frame_manager: &mut FrameManager, sound_manager: &mut SoundManager) {
    use std::f64::consts::{PI, FRAC_PI_2};
    sound_manager.play(line.x,line.y,sounds::SWORD);

    let n = 16;
    let da = PI/(n as f64);
    for i in 0..n+1 {
        let angle = line.angle - FRAC_PI_2 + (i as f64)*da;
        frame_manager.draw_line(color::RED,line.x,line.y,angle,line.length);
    }
}

fn render_sniper_shoot(line: &Line, frame_manager: &mut FrameManager, sound_manager: &mut SoundManager) {
    sound_manager.play(line.x,line.y,sounds::SNIPER);
    frame_manager.draw_line(color::RED,line.x,line.y,line.angle,line.length);
}

fn render_shotgun_shoot(lines: &Vec<Line>, frame_manager: &mut FrameManager, sound_manager: &mut SoundManager) {
    if let Some(line) = lines.get(0) {
        sound_manager.play(line.x,line.y,sounds::SHOTGUN);
    }
    for line in lines {
        frame_manager.draw_line(color::RED,line.x,line.y,line.angle,line.length);
    }
}

fn render_rifle_shoot(line: &Line, frame_manager: &mut FrameManager, sound_manager: &mut SoundManager) {
    sound_manager.play(line.x,line.y,sounds::RIFLE);
    frame_manager.draw_line(color::RED,line.x,line.y,line.angle,line.length);
}
