use frame_manager::{color, FrameManager, Animation};
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
    SwordAttack(Position,f64),
    SwordAttack0(Position,f64,usize),
    SwordAttack1(Position,f64,usize),
    SwordAttack2(Position,f64,usize),
    SwordAttack3(Position,f64,usize),

    BoidExplosion(Position),
    BoidExplosion0(Position,usize),
    BoidExplosion1(Position,usize),
    BoidExplosion2(Position,usize),
    BoidExplosion3(Position,usize),

    WaspAttack(Position),
    WaspDeath(Position),

    SniperShoot(Line),
    ShotgunShoot(Vec<Line>),
    RifleShoot(Line),
    BurningWallDecision(Position),
    GrenadeExplosion(Vec<Line>),
}

pub struct EffectManager {
    effects: Vec<Effect>,
}

pub const DELTA_TIME: usize = 1;

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
        let mut vec = Vec::new();
        while let Some(effect) = self.effects.pop() {
            match effect {
                Effect::SwordAttack(position,angle) => {
                    sound_manager.play(position.x,position.y,sounds::SWORD);
                    render_sword_attack(position.x,position.y,angle,0,frame_manager);
                    vec.push(Effect::SwordAttack0(position,angle,1));
                },
                Effect::SwordAttack0(position,angle,counter) => {
                    render_sword_attack(position.x,position.y,angle,0,frame_manager);
                    if counter >= DELTA_TIME {
                        vec.push(Effect::SwordAttack1(position,angle,0));
                    } else {
                        vec.push(Effect::SwordAttack0(position,angle,counter+1));
                    }
                },
                Effect::SwordAttack1(position,angle,counter) => {
                    render_sword_attack(position.x,position.y,angle,1,frame_manager);
                    if counter >= DELTA_TIME {
                        vec.push(Effect::SwordAttack2(position,angle,0));
                    } else {
                        vec.push(Effect::SwordAttack1(position,angle,counter+1));
                    }
                },
                Effect::SwordAttack2(position,angle,counter) => {
                    render_sword_attack(position.x,position.y,angle,2,frame_manager);
                    if counter >= DELTA_TIME {
                        vec.push(Effect::SwordAttack3(position,angle,0));
                    } else {
                        vec.push(Effect::SwordAttack2(position,angle,counter+1));
                    }
                },
                Effect::SwordAttack3(position,angle,counter) => {
                    render_sword_attack(position.x,position.y,angle,3,frame_manager);
                    if counter >= DELTA_TIME {
                    } else {
                        vec.push(Effect::SwordAttack3(position,angle,counter+1));
                    }
                },

                Effect::BoidExplosion(position) => {
                    sound_manager.play(position.x,position.y,sounds::BOID_EXPLOSION);
                    render_boid_explosion(position.x,position.y,0,frame_manager);
                    vec.push(Effect::BoidExplosion0(position,1));
                },
                Effect::BoidExplosion0(position,counter) => {
                    render_boid_explosion(position.x,position.y,0,frame_manager);
                    if counter >= DELTA_TIME {
                        vec.push(Effect::BoidExplosion1(position,0));
                    } else {
                        vec.push(Effect::BoidExplosion0(position,counter+1));
                    }
                },
                Effect::BoidExplosion1(position,counter) => {
                    render_boid_explosion(position.x,position.y,1,frame_manager);
                    if counter >= DELTA_TIME {
                        vec.push(Effect::BoidExplosion2(position,0));
                    } else {
                        vec.push(Effect::BoidExplosion1(position,counter+1));
                    }
                },
                Effect::BoidExplosion2(position,counter) => {
                    render_boid_explosion(position.x,position.y,2,frame_manager);
                    if counter >= DELTA_TIME {
                        vec.push(Effect::BoidExplosion3(position,0));
                    } else {
                        vec.push(Effect::BoidExplosion2(position,counter+1));
                    }
                },
                Effect::BoidExplosion3(position,counter) => {
                    render_boid_explosion(position.x,position.y,3,frame_manager);
                    if counter >= DELTA_TIME {
                    } else {
                        vec.push(Effect::BoidExplosion3(position,counter+1));
                    }
                },

                Effect::WaspAttack(position) => sound_manager.play(position.x,position.y,sounds::WASP_ATTACK),
                Effect::WaspDeath(position) => sound_manager.play(position.x,position.y,sounds::WASP_DEATH),
                Effect::SniperShoot(line) => render_sniper_shoot(&line,frame_manager,sound_manager),
                Effect::ShotgunShoot(lines) => render_shotgun_shoot(&lines,frame_manager,sound_manager),
                Effect::RifleShoot(line) => render_rifle_shoot(&line,frame_manager,sound_manager),
                Effect::BurningWallDecision(position) => sound_manager.play(position.x,position.y,sounds::BURNING_WALL),
                Effect::GrenadeExplosion(lines) => render_grenade_explosion(&lines,frame_manager,sound_manager),
            }
        }
        self.effects = vec;
    }

}

fn render_sword_attack(x: f64, y: f64, angle: f64, state: usize, frame_manager: &mut FrameManager) {
    match state {
        0 => frame_manager.draw_animation(x,y,angle,Animation::SwordAttack0),
        1 => frame_manager.draw_animation(x,y,angle,Animation::SwordAttack1),
        2 => frame_manager.draw_animation(x,y,angle,Animation::SwordAttack2),
        _ => frame_manager.draw_animation(x,y,angle,Animation::SwordAttack3),
    }
}

fn render_boid_explosion(x: f64, y: f64, state: usize, frame_manager: &mut FrameManager) {
    match state {
        0 => frame_manager.draw_animation(x,y,0.,Animation::BoidExplosion0),
        1 => frame_manager.draw_animation(x,y,0.,Animation::BoidExplosion1),
        2 => frame_manager.draw_animation(x,y,0.,Animation::BoidExplosion2),
        _ => frame_manager.draw_animation(x,y,0.,Animation::BoidExplosion3),
    }
}

fn render_sniper_shoot(line: &Line, frame_manager: &mut FrameManager, sound_manager: &mut SoundManager) {
    sound_manager.play(line.x,line.y,sounds::SNIPER);
    frame_manager.draw_line(color::BLACK,line.x,line.y,line.angle,line.length);
}

fn render_shotgun_shoot(lines: &Vec<Line>, frame_manager: &mut FrameManager, sound_manager: &mut SoundManager) {
    if let Some(line) = lines.get(0) {
        sound_manager.play(line.x,line.y,sounds::SHOTGUN);
    }
    for line in lines {
        frame_manager.draw_line(color::BLACK,line.x,line.y,line.angle,line.length);
    }
}

fn render_rifle_shoot(line: &Line, frame_manager: &mut FrameManager, sound_manager: &mut SoundManager) {
    sound_manager.play(line.x,line.y,sounds::RIFLE);
    frame_manager.draw_line(color::BLACK,line.x,line.y,line.angle,line.length);
}

fn render_grenade_explosion(lines: &Vec<Line>, frame_manager: &mut FrameManager, sound_manager: &mut SoundManager) {
    if let Some(line) = lines.get(0) {
        sound_manager.play(line.x,line.y,sounds::GRENADE_EXPLOSION);
    }
    for line in lines {
        frame_manager.draw_line(color::BLACK,line.x,line.y,line.angle,line.length);
    }
}
