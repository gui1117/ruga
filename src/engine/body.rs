use utils;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PhysicType {
    Static,
    Kinetic,
    Dynamic,
}

#[derive(Clone,Copy,Debug)]
pub enum CollisionBehavior {
    Persist,
    Stop,
    Bounce,
}

pub struct Location {
    pub up: f64,
    pub down: f64,
    pub left: f64,
    pub right: f64,
}

pub type Flags = u64;

pub struct Body {
    pub id: usize,
    pub life: f64,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub weight: f64,
    pub velocity: f64,
    pub angle: f64,
    pub mask: Flags,
    pub group: Flags,
    pub collision_behavior: CollisionBehavior,
    pub physic_type: PhysicType,
}

impl Body {
    pub fn damage(&mut self, d: f64) {
        self.life -= d;
    }

    pub fn up (&self) -> f64 {
        self.y + self.height/2.
    }
    pub fn down (&self) -> f64 {
        self.y - self.height/2.
    }
    pub fn left (&self) -> f64 {
        self.x - self.width/2.
    }
    pub fn right (&self) -> f64 {
        self.x + self.width/2.
    }

    pub fn collide(&self, other: &Body) -> bool {
        let a = self;
        let b = other;
        if (a.group & b.mask != 0) && (b.group & a.mask != 0) {
            let a_min_x = a.x - a.width/2.;
            let a_max_x = a.x + a.width/2.;
            let a_min_y = a.y - a.height/2.;
            let a_max_y = a.y + a.height/2.;
            let b_min_x = b.x - b.width/2.;
            let b_max_x = b.x + b.width/2.;
            let b_min_y = b.y - b.height/2.;
            let b_max_y = b.y + b.height/2.;

            if (a_min_x >= b_max_x) || (b_min_x >= a_max_x) || (a_min_y >= b_max_y) || (b_min_y >= a_max_y) {
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn resolve_collision(&mut self, other: &Body) {
        use std::f64::consts::PI;

        if self.physic_type != PhysicType::Dynamic {
            return;
        }

        let a = self;
        let b = other;

        let hori_dir;
        let delta;

        {
            let a_min_x = a.x - a.width/2.;
            let a_max_x = a.x + a.width/2.;
            let a_min_y = a.y - a.height/2.;
            let a_max_y = a.y + a.height/2.;
            let b_min_x = b.x - b.width/2.;
            let b_max_x = b.x + b.width/2.;
            let b_min_y = b.y - b.height/2.;
            let b_max_y = b.y + b.height/2.;

            let delta_ox = b_max_x - a_min_x;
            let delta_oxp = b_min_x - a_max_x;
            let delta_oy = b_max_y - a_min_y;
            let delta_oyp =  b_min_y - a_max_y;

            let delta_x;
            if delta_ox.abs() < delta_oxp.abs() {
                delta_x = delta_ox;
            } else {
                delta_x = delta_oxp;
            }

            let delta_y;
            if delta_oy.abs() < delta_oyp.abs() {
                delta_y = delta_oy;
            } else {
                delta_y = delta_oyp;
            }

            if delta_x.abs() < delta_y.abs() {
                hori_dir = true;
                delta = delta_x;
            } else {
                hori_dir = false;
                delta = delta_y;
            }
        }

        let mut rate = a.weight/(a.weight+b.weight);
        if rate.is_nan() {
            rate = 1.;
        }

        if hori_dir {
            a.x += (1.-rate)*delta;
        } else {
            a.y += (1.-rate)*delta;
        }
        match a.collision_behavior {
            CollisionBehavior::Bounce => {
                a.angle = if hori_dir {
                    PI - a.angle
                } else {
                    - a.angle
                };
            },
            CollisionBehavior::Stop => {a.velocity = 0.;},
            CollisionBehavior::Persist => (),
        }
    }

    pub fn location(&self) -> Location {
        Location {
            up: self.y + self.height/2.,
            down: self.y - self.height/2.,
            left: self.x - self.width/2.,
            right: self.x + self.width/2.,
        }
    }

    pub fn raycast(&self, a: f64, b: f64, c: f64) -> Option<(f64,f64,f64,f64)> {
        utils::bounding_box_raycast(self.x,self.y,self.width,self.height,a,b,c)
    }

    pub fn in_circle(&self, origin: [f64;2], radius: f64) -> bool {
        let trans = vec![
            [self.left()-origin[0],self.down()-origin[1]],
            [self.left()-origin[0],self.up()-origin[1]],
            [self.right()-origin[0],self.down()-origin[1]],
                [self.right()-origin[0],self.up()-origin[1]]
        ];

        let alpha = {
            let mut index = 0;
            let mut min = trans[0][0].powi(2) + trans[0][1].powi(2);
            for p in 1..4 {
                let d = trans[p][0].powi(2) + trans[p][1].powi(2);
                if d < min {
                    min = d;
                    index = p
                }
            }

            trans[index][1].atan2(trans[index][0])
        };

        let mut projections = Vec::new();
        for p in trans {
            let proj_x = p[0]*alpha.cos() + p[1]*alpha.sin();
            let proj_y = -p[0]*alpha.sin() + p[1]*alpha.cos();

            projections.push([proj_x,proj_y]);
        }
        let mut min_x = projections[0][0];
        let mut max_x = projections[0][0];
        let mut min_y = projections[0][1];
        let mut max_y = projections[0][1];
        for i in 1..4 {
            let x = projections[i][0];
            let y = projections[i][1];
            if x < min_x {
                min_x = x;
            } else if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            } else if y > max_y {
                max_y = y;
            }
        }

        if (min_x >= radius) || (-radius >= max_x) || (min_y >= radius) || (-radius >= max_y) {
            false
        } else {
            true
        }
    }

    pub fn in_location(&self,loc: &Location) -> bool {
        if (self.left() >= loc.right) || (loc.left >= self.right()) || (self.down() >= loc.up) || (loc.down >= self.up()) {
            false
        } else {
            true
        }
    }

    //pub pub fn delta_snapshot(&mut self) -> Option<BodySnapshot>;

    //pub pub fn interpolate(&mut self, from: &BodySnapshot, to: &BodySnapshot, percent: f64);
}
