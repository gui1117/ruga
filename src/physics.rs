pub const PHYSIC_RATE: f32 = 0.9;

pub fn compute_force_damping(velocity: f32, time_to_reach_v_max: f32, weight: f32) -> (f32, f32) {
    let damping = -weight * (1. - PHYSIC_RATE).ln() / time_to_reach_v_max;
    let force = velocity * damping;
    (force, damping)
}

pub struct Resolution {
    pub dx: f32,
    pub dy: f32,
}

impl Resolution {
    pub fn none() -> Resolution {
        Resolution {
            dx: 0.,
            dy: 0.,
        }
    }
    pub fn push(&mut self, res: Resolution) {
        if res.dx.abs() > self.dx.abs() { self.dx = res.dx; }
        if res.dy.abs() > self.dy.abs() { self.dy = res.dy; }
    }
}

#[derive(Clone)]
pub struct RayCast {
    pub origin: [f32; 2],
    pub angle: f32,
    pub length: f32,
    pub mask: u32,
    pub group: u32,
    pub not: Vec<::specs::Entity>,
}

pub enum ContinueOrStop {
    Continue,
    Stop,
}

#[derive(Clone)]
pub struct ShapeCast {
    pub pos: [f32; 2],
    pub shape: Shape,
    pub mask: u32,
    pub group: u32,
    pub not: Vec<::specs::Entity>,
}

/// if A collide with B then collision must represent
/// the smallest vector to move A so it doesn't collide anymore
pub struct Collision {
    pub delta_x: f32,
    pub delta_y: f32,
}
impl Collision {
    pub fn opposite(&self) -> Collision {
        Collision {
            delta_x: -self.delta_x,
            delta_y: -self.delta_y,
        }
    }
}

#[derive(Clone)]
pub enum Shape {
    /// radius
    Circle(f32),
    /// width and height
    Rectangle(f32, f32),
}
impl Shape {
    pub fn cells(&self, pos: [f32; 2]) -> Vec<[i32; 2]> {
        use ::std::f32::EPSILON;

        let (w2, h2) = match *self {
            Shape::Circle(r) => (r, r),
            Shape::Rectangle(w, h) => (w / 2., h / 2.),
        };

        let min_x = (pos[0] - w2 + EPSILON).floor() as i32;
        let max_x = (pos[0] + w2 - EPSILON).floor() as i32;
        let min_y = (pos[1] - h2 + EPSILON).floor() as i32;
        let max_y = (pos[1] + h2 - EPSILON).floor() as i32;

        let mut cells = Vec::new();
        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                cells.push([x, y]);
            }
        }
        cells
    }
    pub fn raycast(&self, pos: [f32; 2], eq: (f32, f32, f32)) -> Option<(f32, f32, f32, f32)> {
        use self::Shape::*;
        let (a, b, c) = eq;
        match *self {
            Circle(r) => circle_raycast(pos[0], pos[1], r, a, b, c),
            Rectangle(w, h) => bounding_box_raycast(pos[0], pos[1], w, h, a, b, c),
        }
    }
}

#[derive(Clone)]
pub enum CollisionBehavior {
    #[allow(dead_code)]
    Bounce,
    #[allow(dead_code)]
    Back,
    #[allow(dead_code)]
    Persist,
    #[allow(dead_code)]
    Stop,
}

#[derive(Clone)]
pub struct EntityInformation {
    pub entity: ::specs::Entity,
    pub pos: [f32; 2],
    pub group: u32,
    pub mask: u32,
    pub shape: Shape,
}

pub fn grid_raycast(x0: f32, y0: f32, x1: f32, y1: f32) -> Vec<[i32; 2]> {
    if (x1 - x0).abs() < (y1 - y0).abs() {
        grid_raycast(y0, x0, y1, x1).iter().map(|s| [s[1], s[0]]).collect::<Vec<[i32; 2]>>()
    } else if x0 > x1 {
        let mut vec = grid_raycast(x1, y1, x0, y0);
        vec.reverse();
        vec
    } else {
        let x0_i32 = x0.floor() as i32;
        let y0_i32 = y0.floor() as i32;
        let x1_i32 = x1.floor() as i32;

        // equation y = ax + b
        let a = (y1 - y0) / (x1 - x0);
        let b = y0 - a * x0;

        let delta_error = a.abs();
        let signum = a.signum() as i32;

        let mut error = if a > 0. {
            (a * x0.floor() + b) - y0.floor()
        } else {
            y0.ceil() - (a * x0.floor() + b)
        };


        let mut vec = Vec::new();
        let mut y = y0_i32;

        for x in x0_i32..x1_i32 + 1 {
            error += delta_error;
            vec.push([x, y]);
            while error >= 1.0 {
                y += signum;
                error -= 1.0;
                vec.push([x, y]);
            }
        }
        vec
    }
}

/// the coordinate of the intersections (if some) of a circle of center (x,y) and radius,
/// and the line of equation ax+by+c=0
fn circle_raycast(x: f32, y: f32, radius: f32, a: f32, b: f32, c: f32) -> Option<(f32, f32, f32, f32)> {
    use ::std::f32::EPSILON;
    // println!("x:{}, y:{}, radius:{}, a:{}, b:{}, c:{}",x,y,radius,a,b,c);
    if a == 0. && b == 0. {
        panic!("invalid line equation")
    } else if (a / radius).abs() < EPSILON {
        let y_ray = -c / b;
        if (y_ray - y).abs() < radius {
            let dx = (radius.powi(2) - (y_ray - y).powi(2)).sqrt();
            Some((x - dx, y_ray, x + dx, y_ray))
        } else {
            None
        }
    } else if (b / radius).abs() < EPSILON {
        let x_ray = -c / a;
        if (x_ray - x).abs() < radius {
            let dy = (radius.powi(2) - (x_ray - x).powi(2)).sqrt();
            Some((x_ray, y - dy, x_ray, y + dy))
        } else {
            None
        }
    } else {
        // the equation of intersection abscisse: d*x^2 + e*x + f = 0
        let d = 1. + (a / b).powi(2);
        let e = 2. * (-x + a / b * (c / b + y));
        let f = x.powi(2) + (c / b + y).powi(2) - radius.powi(2);

        let delta = e.powi(2) - 4. * d * f;

        if delta > 0. {
            let (x1, x2) = {
                let x1 = (-e - delta.sqrt()) / (2. * d);
                let x2 = (-e + delta.sqrt()) / (2. * d);
                if x1 > x2 { (x2, x1) } else { (x1, x2) }
            };
            let y1 = (-c - a * x1) / b;
            let y2 = (-c - a * x2) / b;

            Some((x1, y1, x2, y2))
        } else {
            None
        }
    }
}

/// The line of equation ax + by + c = 0 that pass through the two points
fn line_equation_from_points(p: [f32; 2], q: [f32; 2]) -> (f32, f32, f32) {
    let (a, b) = if (p[0] - q[0]).abs() > (p[1] - q[1]).abs() {
        (- (p[1] - q[1]) / (p[0] - q[0]), 1.)
    } else {
        (1., - (p[0] - q[0]) / (p[1] - q[1]))
    };
    let c = - a*p[0] - b*p[1];

    (a, b, c)
}

/// the coordinate of the intersections (if some) of a rectangle of center (x,y) width and height,
/// and the line of equation ax+by+c=0
fn bounding_box_raycast(x: f32, y: f32, width: f32, height: f32, a: f32, b: f32, c: f32) -> Option<(f32, f32, f32, f32)> {
    if a == 0. && b == 0. {
        panic!("invalid line equation")
    } else if a == 0. {
        let y_proj = -c / b;
        if y - height / 2. <= y_proj && y_proj <= y + height / 2. {
            Some((x - width / 2., y_proj, x + width / 2., y_proj))
        } else {
            None
        }
    } else if b == 0. {
        let x_proj = -c / a;
        if x - width / 2. <= x_proj && x_proj <= x + width / 2. {
            Some((x_proj, y - height / 2., x_proj, y + height / 2.))
        } else {
            None
        }
    } else {
        // println!("x:{}, y:{}, width:{}, height:{}, a:{}, b:{}, c:{}",x,y,width,height,a,b,c);
        // the ordonate of the point that is on the line(a,b,c)
        // and the horizontal line that cut (x,y)
        let y_proj = -(a * x + c) / b;
        // the abscisse of the point that is on the line(a,b,c)
        // and the vertical line that cut (x,y)
        let x_proj = -(b * y + c) / a;

        // println!("proj: {:?} | {:?}",x_proj,y_proj);
        // i,j,k,l are three point:
        // * i represent the point on the horizontal line on the top of the bounding box
        // and on the line(a,b)
        // * j represent the point on the vertical line on the right of the bounding box
        // and on the line(a,b)
        // * k represent the point on the horizontal line on the bottom of the bounding box
        // and on the line(a,b)
        // * l represent the point on the vertical line on the left of the bounding box
        // and on the line(a,b)

        // dy = -a/b * dx

        let dx = -height / 2. * b / a;
        // println!("dx: {:?}",dx);
        let x_i = x_proj + dx;
        let y_i = y + height / 2.;
        let x_k = x_proj - dx;
        let y_k = y - height / 2.;
        // println!("i: {:?} | {:?}",x_i,y_i);
        // println!("k: {:?} | {:?}",x_k,y_k);

        let dy = -width / 2. * a / b;
        // println!("dy: {:?}",dy);
        let x_j = x + width / 2.;
        let y_j = y_proj + dy;
        let x_l = x - width / 2.;
        let y_l = y_proj - dy;
        // println!("j: {:?} | {:?}",x_j,y_j);
        // println!("l: {:?} | {:?}",x_l,y_l);


        let cond_i = x - width / 2. <= x_i && x_i <= x + width / 2.;
        let cond_k = x - width / 2. <= x_k && x_k <= x + width / 2.;
        let cond_j = y - height / 2. <= y_j && y_j <= y + height / 2.;
        let cond_l = y - height / 2. <= y_l && y_l <= y + height / 2.;
        // println!("cond i: {}, j: {}, k: {}, l: {}",cond_i,cond_j,cond_k,cond_l);

        match (cond_i, cond_k, cond_j, cond_l) {
            (true, true, _, _) => Some((x_i, y_i, x_k, y_k)),
            (true, _, true, _) => Some((x_i, y_i, x_j, y_j)),
            (true, _, _, true) => Some((x_i, y_i, x_l, y_l)),
            (_, true, true, _) => Some((x_j, y_j, x_k, y_k)),
            (_, true, _, true) => Some((x_k, y_k, x_l, y_l)),
            (_, _, true, true) => Some((x_j, y_j, x_l, y_l)),
            _ => None,
        }
    }
}

pub fn shape_collision(a_pos: [f32;2], a_shape: &Shape, b_pos: [f32;2], b_shape: &Shape) -> Option<Collision> {
    use self::Shape::*;
    match (a_shape, b_shape) {
        (&Circle(a_radius), &Circle(b_radius)) => circle_circle_collision(a_pos, a_radius, b_pos, b_radius),
        (&Circle(a_radius), &Rectangle(b_w, b_h)) => circle_rectangle_collision(a_pos, a_radius, b_pos, b_w, b_h),
        (&Rectangle(a_w, a_h), &Rectangle(b_w, b_h)) => rectangle_rectangle_collision(a_pos, a_w, a_h, b_pos, b_w, b_h),
        (&Rectangle(a_w, a_h), &Circle(b_radius)) => circle_rectangle_collision(b_pos, b_radius, a_pos, a_w, a_h).map(|col| col.opposite()),
    }
}

fn circle_circle_collision(a_pos: [f32;2], a_rad: f32, b_pos: [f32;2], b_rad: f32) -> Option<Collision> {
    let dx = a_pos[0]-b_pos[0];
    let dy = a_pos[1]-b_pos[1];
    let dn2 = dx.powi(2) + dy.powi(2);
    let rad = a_rad+b_rad;
    if dn2 < rad.powi(2) {
        let angle = dy.atan2(dx);
        let dn = dn2.sqrt();
        let delta = rad - dn;
        Some(Collision {
            delta_x: delta*angle.cos(),
            delta_y: delta*angle.sin(),
        })
    } else {
        None
    }
}
fn circle_rectangle_collision(a_pos: [f32;2], a_radius: f32, b_pos: [f32;2], b_width: f32, b_height: f32) -> Option<Collision> {
    let left = a_pos[0] < b_pos[0] - b_width/2.;
    let right = a_pos[0] > b_pos[0] + b_width/2.;
    let down = a_pos[1] < b_pos[1] - b_height/2.;
    let up = a_pos[1] > b_pos[1] + b_height/2.;

    let extern_horizontal = left || right;
    let extern_vertical = up || down;

    if extern_horizontal && extern_vertical {
        let insider = [if left { b_pos[0] - b_width/2. } else { b_pos[0] + b_width/2.},
                       if down { b_pos[1] - b_height/2. } else { b_pos[1] + b_height/2.}];

        if (insider[0]-a_pos[0]).powi(2) + (insider[1]-a_pos[1]).powi(2) >= a_radius.powi(2) {
            return None
        }

        let (a, b, c) = line_equation_from_points(insider, a_pos);
        let outsider = if let Some((x0, y0, x1, y1)) = circle_raycast(a_pos[0], a_pos[1], a_radius, a, b, c) {
            [if left { x0.max(x1) } else { x0.min(x1) },
             if down { y0.max(y1) } else {  y0.min(y1) }]
        } else {
            return None
        };


        Some(Collision {
            delta_x: insider[0] - outsider[0],
            delta_y: insider[1] - outsider[1],
        })
    } else {
        rectangle_rectangle_collision(a_pos, a_radius*2., a_radius*2., b_pos, b_width, b_height)
    }
}
fn rectangle_rectangle_collision(a_pos: [f32;2], a_width: f32, a_height: f32, b_pos: [f32;2], b_width: f32, b_height: f32) -> Option<Collision> {
    let a_min_x = a_pos[0] - a_width/2.;
    let a_max_x = a_pos[0] + a_width/2.;
    let a_min_y = a_pos[1] - a_height/2.;
    let a_max_y = a_pos[1] + a_height/2.;
    let b_min_x = b_pos[0] - b_width/2.;
    let b_max_x = b_pos[0] + b_width/2.;
    let b_min_y = b_pos[1] - b_height/2.;
    let b_max_y = b_pos[1] + b_height/2.;

    if (a_min_x >= b_max_x) || (b_min_x >= a_max_x) || (a_min_y >= b_max_y) || (b_min_y >= a_max_y) {
        None
    } else {
        let delta_ox = b_max_x - a_min_x;
        let delta_oxp = b_min_x - a_max_x;
        let delta_oy = b_max_y - a_min_y;
        let delta_oyp =  b_min_y - a_max_y;

        let delta_x = if delta_ox.abs() < delta_oxp.abs() {
            delta_ox
        } else {
            delta_oxp
        };

        let delta_y = if delta_oy.abs() < delta_oyp.abs() {
            delta_oy
        } else {
            delta_oyp
        };

        if delta_x.abs() < delta_y.abs() {
            Some(Collision {
                delta_x: delta_x,
                delta_y: 0.,
            })
        } else {
            Some(Collision {
                delta_x: 0.,
                delta_y: delta_y,
            })
        }
    }
}
#[test]
fn circle_raycast_test() {
    // for a == 0
    assert_eq!(Some((-1., 3., 3., 3.)),
               circle_raycast(1., 3., 2., 0., -1., 3.));

    // for b == 0
    assert_eq!(Some((3., 0., 3., 2.)),
               circle_raycast(3., 1., 1., -1., 0., 3.));

    // for b != 0 && a != 0
    assert_eq!(Some((-0.99999994, -0.99999994, 0.99999994, 0.99999994)),
               circle_raycast(0., 0., 2f32.sqrt(), 1., -1., 0.));
}

#[test]
fn test_bounding_box_raycast() {
    // for a == 0
    assert_eq!(None, bounding_box_raycast(-1., -2., 6., 2., 0., 0., -1.));
    assert_eq!(None, bounding_box_raycast(-1., -2., 6., 2., 0., 1., -1.));
    assert_eq!(None,
               bounding_box_raycast(-1., -2., 6., 2., 0., -1. / 0.5, -1.));
    assert_eq!(Some((-4., -1., 2., -1.)),
               bounding_box_raycast(-1., -2., 6., 2., 0., -1. / 1., -1.));
    assert_eq!(Some((-4., -2., 2., -2.)),
               bounding_box_raycast(-1., -2., 6., 2., 0., -1. / 2., -1.));
    assert_eq!(Some((-4., -3., 2., -3.)),
               bounding_box_raycast(-1., -2., 6., 2., 0., -1. / 3., -1.));
    assert_eq!(None,
               bounding_box_raycast(-1., -2., 6., 2., 0., -1. / 3.5, -1.));
    assert_eq!(None,
               bounding_box_raycast(-1., -2., 6., 2., 0., -1. / 4., -1.));
    assert_eq!(None,
               bounding_box_raycast(-1., -2., 6., 2., 0., -1. / 4.5, -1.));

    // for b == 0
    assert_eq!(None,
               bounding_box_raycast(-1., -2., 6., 2., -1. / 4.5, 0., -1.));
    assert_eq!(Some((-4., -3., -4., -1.)),
               bounding_box_raycast(-1., -2., 6., 2., -1. / 4., 0., -1.));
    assert_eq!(Some((0., -3., 0., -1.)),
               bounding_box_raycast(-1., -2., 6., 2., 1., 0., 0.));
    assert_eq!(Some((2., -3., 2., -1.)),
               bounding_box_raycast(-1., -2., 6., 2., 1. / 2., 0., -1.));
    assert_eq!(None,
               bounding_box_raycast(-1., -2., 6., 2., 1. / 2.5, 0., -1.));

    // for b != 0 && a != 0
    assert_eq!(None, bounding_box_raycast(-1., -2., 6., 2., -1., -1., 9.));
    assert_eq!(None, bounding_box_raycast(-1., -2., 6., 2., 1., 1., 7.));
    assert_eq!(Some((-4., -2., -3., -3.)),
               bounding_box_raycast(-1., -2., 6., 2., 1., 1., 6.));

    assert_eq!(Some((-4., -1.96, 2., -2.02)),
               bounding_box_raycast(-1., -2., 6., 2., 0.01, 1., 2.));
}
