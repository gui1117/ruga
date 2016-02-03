use viewport::Viewport;
use opengl_graphics::GlGraphics;
use rand;
use rand::distributions::{IndependentSample, Range};
use world::{ 
    Camera, 
    World,
};
use world::spatial_hashing::Location;

use super::{ 
    Body, 
    BodyTrait, 
    BodyType,
    CollisionBehavior,
};
use world::batch::Batch;
use std::rc::Rc;
use std::cell::RefCell;
use std::f64;
use util::minus_pi_pi;

pub struct Character {
    body: Body,
    aim: f64,
    gun: Gun,
    sword: Sword,
    world_batch: Rc<RefCell<Batch>>,
}

pub const WIDTH: f64 = 1.;
pub const VELOCITY: f64 = 65.;
pub const HEIGHT: f64 = 1.;
pub const WEIGHT: f64 = 1.;
pub const MASK: u32 = !0;
pub const GROUP: u32 = 2;


impl Character {
    pub fn new(id: usize, x: f64, y: f64, angle: f64, batch: Rc<RefCell<Batch>>) -> Character {
        Character {
            body: Body {
                id: id,
                x: x,
                y: y,
                width2: WIDTH/2.,
                height2: HEIGHT/2.,
                weight: WEIGHT,
                velocity: 0.,
                angle: angle,
                mask: MASK,
                group: GROUP,
                collision_behavior: CollisionBehavior::Persist,
                body_type: BodyType::Character,
            },
            aim: angle,
            gun: Gun::new(),
            sword: Sword::new(),
            world_batch: batch,
        }
    }
}

struct SwordAttack {
    x: f64,
    y: f64,
    aim: f64,
}

pub const SWORD_RECOVER: f64 = 0.8;
pub const SWORD_LENGTH: f64 = 5.;
pub const SWORD_DAMAGE: f64 = 5.;

struct Sword {
    recover: f64,
    attacks: Vec<SwordAttack>,
}

impl Sword {
    fn new() -> Sword {
        Sword {
            recover: 0.,
            attacks: Vec::new(),
        }
    }
}

trait SwordManager {
    fn sword_attack(&self);
    fn sword_update(&self, dt: f64);
    fn sword_render_debug(&self, lines: &mut Vec<[f64;4]>);
}

impl SwordManager for RefCell<Character> {
    fn sword_update(&self, dt: f64) {
        let recover = self.borrow().sword.recover;
        if recover > 0. {
            self.borrow_mut().sword.recover = (recover - dt).max(0.);
        }
    }
    fn sword_attack(&self) {
        use std::f64::consts::{PI, FRAC_PI_2};

        if self.borrow().sword.recover <= 0. {
            self.borrow_mut().sword.recover = SWORD_RECOVER;

            let (id,x,y,aim) = (self.id(),self.x(), self.y(), minus_pi_pi(self.aim()));
            let batch = self.borrow().world_batch.clone();
            let loc = Location {
                up: y + SWORD_LENGTH,
                down: y - SWORD_LENGTH,
                left: x - SWORD_LENGTH,
                right: x + SWORD_LENGTH,
            };


            batch.borrow().apply_locally(&loc, &mut |body: &Rc<BodyTrait>| {
                if body.id() != id && body.in_circle([x,y],SWORD_LENGTH) {
                    let in_part = if aim == PI {
                        body.left() <= x
                    } else if aim > FRAC_PI_2 {
                        let t_x = body.left() - x;
                        let t_y = body.up() - y;
                        let a = aim - FRAC_PI_2;
                        t_y >= a * t_x
                    } else if aim == FRAC_PI_2 {
                        body.up() >= y
                    } else if aim > 0. {
                        let t_x = body.right() - x;
                        let t_y = body.up() - y;
                        let a = aim - FRAC_PI_2;
                        t_y >= a * t_x
                    } else  if aim == 0. {
                        body.right() >= x
                    } else if aim > -FRAC_PI_2 {
                        let t_x = body.right() - x;
                        let t_y = body.down() - y;
                        let a = aim - FRAC_PI_2;
                        t_y >= a * t_x
                    } else if aim == -FRAC_PI_2 {
                        body.down() <= y
                    } else {
                        let t_x = body.left() - x;
                        let t_y = body.down() - y;
                        let a = aim - FRAC_PI_2;
                        t_y >= a * t_x
                    };

                    if in_part {
                        body.damage(SWORD_DAMAGE);
                    }
                }
            });

            self.borrow_mut().sword.attacks.push(SwordAttack {
                x: x,
                y: y,
                aim: aim,
            });
        }
    }
    fn sword_render_debug(&self, lines: &mut Vec<[f64;4]>) {
        use std::f64::consts::{PI, FRAC_PI_2};

        let ref mut attacks = self.borrow_mut().sword.attacks;
        let n = 16;
        let da = PI/(n as f64);
        while let Some(a) = attacks.pop() {
            for i in 0..n+1 {
                let angle = a.aim - FRAC_PI_2 + (i as f64)*da;
                lines.push([a.x,a.y,a.x+SWORD_LENGTH*angle.cos(),a.y+SWORD_LENGTH*angle.sin()]);
            }
        }
    }
}

#[derive(Clone,Copy,PartialEq)]
pub enum GunType {
    None,
    Rifle,
    Shotgun,
    Sniper,
}

pub enum GunShoot {
    Rifle(f64,f64,f64,f64),
    Sniper(f64,f64,f64,f64),
    Shotgun(f64,f64,f64,f64),
}

struct Gun {
    gun_type: GunType,
    next_type: GunType,
    reloading: f64,
    shooting: bool,
    ammo: u32,
    shoots: Vec<GunShoot>,
}

pub const RIFLE_RELOADING_TIME: f64 = 0.1;
pub const SHOTGUN_RELOADING_TIME: f64 = 0.8;
pub const SNIPER_RELOADING_TIME: f64 = 1.5;

pub const RIFLE_LENGTH: f64 = 30.;
pub const SHOTGUN_LENGTH: f64 = 30.;
pub const SNIPER_LENGTH: f64 = 70.;

pub const RIFLE_DAMAGE: f64 = 10.;
pub const SHOTGUN_DAMAGE: f64 = 10.;
pub const SNIPER_DAMAGE: f64 = 100.;

pub const RIFLE_MAX_DELTA_ANGLE: f64 = f64::consts::PI/16.;
pub const SHOTGUN_MAX_DELTA_ANGLE: f64 = f64::consts::PI/6.;
pub const SHOTGUN_SHOOT_NUMBER: u32 = 4;

impl Gun {
    pub fn new() -> Gun {
        Gun {
            gun_type: GunType::Rifle,
            next_type: GunType::Rifle,
            shooting: false,
            reloading: 0.,
            ammo: 10000000,
            shoots: Vec::new(),
        }
    }

    pub fn time_to_reload(&mut self) -> f64 {
        match self.gun_type {
            GunType::Sniper => SNIPER_RELOADING_TIME,
            GunType::Shotgun => SHOTGUN_RELOADING_TIME,
            GunType::Rifle => RIFLE_RELOADING_TIME,
            GunType::None => 0.,
        }
    }
}

trait GunManager {
    fn gun_shoot(&self);
    fn gun_update(&self, dt: f64);
    fn gun_render_debug(&self, lines: &mut Vec<[f64;4]>);
}

impl GunManager for RefCell<Character> {
    fn gun_shoot(&self) {
        let (id,x,y,aim,batch,gun_type) = {
            let this = self.borrow();
            (this.body.id,this.body.x,this.body.y,this.aim,this.world_batch.clone(),this.gun.gun_type)
        };
        match gun_type {
            GunType::Sniper => {
                let mut length = SNIPER_LENGTH;
                batch.borrow().raycast(x,y,aim,SNIPER_LENGTH, &mut |body,min,_| {
                    if body.id() != id {
                        if let BodyType::Wall = body.body_type() {
                            length = min;
                            true
                        } else {
                            body.damage(SNIPER_DAMAGE);
                            false
                        }
                    } else {
                        false
                    }
                });
                self.borrow_mut().gun.shoots.push(GunShoot::Sniper(x,y,aim,length));
            },
            GunType::Shotgun => {
                let range = Range::new(-SHOTGUN_MAX_DELTA_ANGLE,SHOTGUN_MAX_DELTA_ANGLE);
                let mut rng = rand::thread_rng();
                for _ in 0..SHOTGUN_SHOOT_NUMBER {
                    let aim = aim + range.ind_sample(&mut rng);
                    let mut length = SHOTGUN_LENGTH;
                    batch.borrow().raycast(x,y,aim,SHOTGUN_LENGTH, &mut |body,min,_| {
                        if body.id() != id {
                            body.damage(SHOTGUN_DAMAGE);
                            length = min;
                            true
                        } else {
                            false
                        }
                    });
                    self.borrow_mut().gun.shoots.push(GunShoot::Shotgun(x,y,aim,length));
                }
            },
            GunType::Rifle => {
                let range = Range::new(-RIFLE_MAX_DELTA_ANGLE,RIFLE_MAX_DELTA_ANGLE);
                let mut rng = rand::thread_rng();
                let aim = aim + range.ind_sample(&mut rng);
                let mut length = RIFLE_LENGTH;
                batch.borrow().raycast(x,y,aim,RIFLE_LENGTH, &mut |body,min,_| {
                    if body.id() != id {
                        body.damage(RIFLE_DAMAGE);
                        length = min;
                        true
                    } else {
                        false
                    }
                });
                self.borrow_mut().gun.shoots.push(GunShoot::Rifle(x,y,aim,length));
            },
            GunType::None => (),
        }
    }

    fn gun_update(&self, dt: f64) {
        {
            let current_type = self.borrow().gun.gun_type;
            let next_type = self.borrow().gun.next_type;
            if next_type != current_type {
                let loc = self.location();
                let batch = self.borrow().world_batch.clone();
                let mut on_armory = false;
                batch.borrow().apply_locally(&loc, &mut |body: &Rc<BodyTrait>| {
                    if body.body_type() == BodyType::Armory {
                        on_armory = true;
                    }
                });
                if on_armory {
                    self.borrow_mut().gun.gun_type = next_type;
                }
            }
        }

        let mut shoot = false;
        {
            let mut this = self.borrow_mut();
            if this.gun.ammo > 0 {
                if this.gun.reloading > 0. {
                    if this.gun.shooting {
                        this.gun.reloading -= dt;
                    } else {
                        let t = this.gun.reloading - dt;
                        this.gun.reloading = t.max(0.);
                    }
                } else if this.gun.shooting {
                    shoot = true;
                    this.gun.ammo -= 1;
                    this.gun.reloading += this.gun.time_to_reload();
                }
            }
        }
        if shoot {
            self.gun_shoot();
        }
    }

    fn gun_render_debug(&self, lines: &mut Vec<[f64;4]>) {
        let ref mut shoots = self.borrow_mut().gun.shoots;
        while let Some(shoot) = shoots.pop() {
            match shoot {
                GunShoot::Sniper(x,y,aim,length)
                | GunShoot::Shotgun(x,y,aim,length)
                | GunShoot::Rifle(x,y,aim,length) => {
                    lines.push([x,y,x+length*aim.cos(),y+length*aim.sin()]);
                },
            }
        }
    }
}

const GRENADE_DISTANCE: f64 = 5.;

pub trait CharacterTrait {
    fn aim(&self) -> f64;
    fn set_aim(&self, a: f64);
    fn set_gun_shoot(&self,bool);
    fn do_sword_attack(&self);
    fn set_next_gun_type(&self, next_type: GunType);
    fn launch_grenade(&self,&mut World);
}

impl CharacterTrait for RefCell<Character> {
    fn aim(&self) -> f64 {
        self.borrow().aim
    }

    fn set_aim(&self, a: f64) {
        self.borrow_mut().aim = a;
    }

    fn set_gun_shoot(&self, shoot: bool) {
        self.borrow_mut().gun.shooting = shoot;
    }

    fn set_next_gun_type(&self, next_type: GunType) {
        self.borrow_mut().gun.next_type = next_type;
    }

    fn do_sword_attack(&self) {
        self.sword_attack();
    }

    fn launch_grenade(&self,world: &mut World) {
        let aim = self.aim();
        let x = self.x() + GRENADE_DISTANCE*aim.cos();
        let y = self.y() + GRENADE_DISTANCE*aim.sin();
        world.insert_grenade(x,y,aim);
    }
}

impl BodyTrait for RefCell<Character> {
    delegate!{
        body:
            id() -> usize,
            dead() -> bool,
            body_type() -> BodyType,
            damage(d: f64) -> (),
            width2() -> f64,
            height2() -> f64,
            x() -> f64,
            mut set_x(x: f64) -> (),
            y() -> f64,
            mut set_y(y: f64) -> (),
            weight() -> f64,
            velocity() -> f64,
            mut set_velocity(v: f64) -> (),
            angle() -> f64,
            mut set_angle(a: f64) -> (),
            mask() -> u32,
            group() -> u32,
            collision_behavior() -> CollisionBehavior,
            render(viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) -> (),
            on_collision(other: &BodyTrait) -> (),
    }

    fn render_debug(&self, lines: &mut Vec<[f64;4]>) {
        self.gun_render_debug(lines);
        self.sword_render_debug(lines);
        let this = self.borrow();
        this.body.render_debug(lines);
    }

    fn update(&self, dt: f64) {
        self.sword_update(dt);
        self.gun_update(dt);
        {
            let mut this = self.borrow_mut();
            this.body.update(dt);
        }
    }
}
