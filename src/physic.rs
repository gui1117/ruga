use app;
use specs;
use config;
use specs::Join;
use std::collections::hash_map::{HashMap, Entry};
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;
use std::f32;

pub trait IntoGrid {
    fn into_grid(&self) -> [f32;2];
}

impl IntoGrid for [i32;2] {
    fn into_grid(&self) -> [f32;2] {
        [self[0] as f32 + 0.5, self[1] as f32 + 0.5]
    }
}
impl IntoGrid for [isize;2] {
    fn into_grid(&self) -> [f32;2] {
        [self[0] as f32 + 0.5, self[1] as f32 + 0.5]
    }
}
impl IntoGrid for [f32;2] {
    fn into_grid(&self) -> [f32;2] {
        self.clone()
    }
}

pub struct GridSquare {
    pub position: [f32;2],
}
impl specs::Component for GridSquare {
    type Storage = specs::VecStorage<Self>;
}
impl GridSquare {
    pub fn new<T: IntoGrid>(pos: T) -> Self {
        GridSquare {
            position: pos.into_grid(),
        }
    }
}

#[derive(Debug,Clone)]
pub enum Shape {
    Circle(f32),
    Square(f32),
}

#[derive(Debug,Clone)]
pub enum CollisionBehavior {
    Bounce,
    Back,
    Persist,
    Stop,
}

#[derive(Debug,Clone)]
pub struct PhysicTrigger {
    pub active: bool,
}
impl specs::Component for PhysicTrigger {
    type Storage = specs::VecStorage<Self>;
}
impl PhysicTrigger {
    pub fn new() -> Self {
        PhysicTrigger {
            active: false,
        }
    }
}

#[derive(Debug,Clone)]
pub struct PhysicState {
    pub position: [f32;2],
    pub velocity: [f32;2],
    pub acceleration: [f32;2],
}
impl PhysicState {
    pub fn new<T: IntoGrid>(pos: T) -> Self {
        PhysicState{
            position: pos.into_grid(),
            velocity: [0.,0.],
            acceleration: [0.,0.],
        }
    }
}
impl specs::Component for PhysicState {
    type Storage = specs::VecStorage<Self>;
}

#[derive(Debug,Clone)]
pub struct PhysicType {
    pub shape: Shape,
    pub collision_behavior: CollisionBehavior,
    pub damping: f32,
    pub force: f32,
    pub weight: f32,
    pub group: u32,
    pub mask: u32,
}
impl PhysicType {
    pub fn new_movable(group: u32, mask: u32, shape: Shape, collision: CollisionBehavior, velocity: f32, time_to_reach_v_max: f32, weight: f32) -> Self {
        let damping = -weight * (1.-config.physic.rate).ln() / time_to_reach_v_max;
        let force = velocity * damping;
        PhysicType {
            shape: shape,
            collision_behavior: collision,
            weight: weight,
            damping: damping,
            force: force,
            group: group,
            mask: mask,
        }
    }
    pub fn new_static(group: u32, mask: u32, shape: Shape) -> Self {
        PhysicType {
            shape: shape,
            collision_behavior: CollisionBehavior::Persist,
            weight: f32::MAX,
            force: 0.,
            damping: 0.,
            group: group,
            mask: mask,
        }
    }
}
impl specs::Component for PhysicType {
    type Storage = specs::VecStorage<Self>;
}

#[derive(Debug,Clone)]
pub struct PhysicForce {
    pub direction: f32,
    pub intensity: f32,
}
impl PhysicForce {
    pub fn new() -> Self {
        PhysicForce {
            direction: 0.,
            intensity: 0.,
        }
    }
    pub fn new_full() -> Self {
        PhysicForce {
            direction: 0.,
            intensity: 1.,
        }
    }
}
impl specs::Component for PhysicForce {
    type Storage = specs::VecStorage<Self>;
}

#[derive(Debug,Clone,Default)]
pub struct PhysicDynamic;
impl specs::Component for PhysicDynamic {
    type Storage = specs::NullStorage<Self>;
}

#[derive(Debug,Clone,Default)]
pub struct PhysicStatic;
impl specs::Component for PhysicStatic {
    type Storage = specs::NullStorage<Self>;
}

#[derive(Debug,Clone)]
pub struct Ray {
    pub origin: [f32;2],
    pub angle: f32,
    pub length: f32,
    pub mask: u32,
}

/// if A collide with B then collision must represent
/// the smallest vector to move A so it doesn't collide anymore
pub struct Collision {
    delta_x: f32,
    delta_y: f32,
}

pub struct PhysicWorld {
    unit: f32,
    static_hashmap: HashMap<[i32;2],Vec<(specs::Entity,[f32;2],u32,Shape)>,BuildHasherDefault<FnvHasher>>,
    movable_hashmap: HashMap<[i32;2],Vec<(specs::Entity,[f32;2],u32,Shape)>,BuildHasherDefault<FnvHasher>>,
}

#[derive(Debug)]
struct Resolution {
    dx: f32,
    dy: f32,
}

impl Resolution {
    fn push(&mut self, res: Resolution) {
        if res.dx.abs() > self.dx.abs() { self.dx = res.dx; }
        if res.dy.abs() > self.dy.abs() { self.dy = res.dy; }
    }
}

pub struct PhysicSystem;
impl specs::System<app::UpdateContext> for PhysicSystem {
    fn run(&mut self, arg: specs::RunArg, context: app::UpdateContext) {
        use std::f32::consts::PI;
        use specs::Join;

        let (dynamics,mut states,forces,types,mut physic_world,mut triggers,entities) = arg.fetch(|world| {
            (
                world.read::<PhysicDynamic>(),
                world.write::<PhysicState>(),
                world.read::<PhysicForce>(),
                world.read::<PhysicType>(),
                world.write_resource::<PhysicWorld>(),
                world.write::<PhysicTrigger>(),
                world.entities(),
            )
        });

        let dt = context.dt as f32;

        let mut resolutions = HashMap::<specs::Entity,Resolution>::new();

        for trigger in (&mut triggers).iter() {
            trigger.active = false;
        }
        let fnv = BuildHasherDefault::<FnvHasher>::default();
        physic_world.movable_hashmap = HashMap::with_hasher(fnv);
        for (_,entity) in (&dynamics, &entities).iter() {
            let state = states.get_mut(entity).expect("dynamic entity expect state component");
            let force = forces.get(entity).expect("dynamic entity expect force component");
            let typ = types.get(entity).expect("dynamic entity expect type component");

            state.acceleration[0] = (typ.force*force.intensity*force.direction.cos()
                                     - typ.damping*state.velocity[0])/typ.weight;

            state.acceleration[1] = (typ.force*force.intensity*force.direction.sin()
                                     - typ.damping*state.velocity[1])/typ.weight;

            state.velocity[0] += dt*state.acceleration[0];
            state.velocity[1] += dt*state.acceleration[1];

            state.position[0] += dt*state.velocity[0];
            state.position[1] += dt*state.velocity[1];

            if typ.mask == 0 { continue }

            physic_world.apply_on_shape(&state.position, typ.mask, &typ.shape, &mut |other_entity,collision| {
                let other_type = types.get(*other_entity).expect("physic entity expect type component");

                if other_type.mask & typ.group != 0 {

                    if let Some(trigger) = triggers.get_mut(entity) {
                        trigger.active = true;
                    }
                    if let Some(trigger) = triggers.get_mut(*other_entity) {
                        trigger.active = true;
                    }

                    let rate = {
                        if other_type.weight == f32::MAX {
                            0.
                        } else if typ.weight == f32::MAX {
                            1.
                        } else {
                            typ.weight/(typ.weight+other_type.weight)
                        }
                    };

                    if rate != 1. {
                        let resolution = Resolution {
                            dx: collision.delta_x*(1.-rate),
                            dy: collision.delta_y*(1.-rate),
                        };
                        match resolutions.entry(entity) {
                            Entry::Occupied(mut entry) => entry.get_mut().push(resolution),
                            Entry::Vacant(entry) => {entry.insert(resolution);},
                        }
                    }
                    if rate != 0. {
                        let resolution = Resolution {
                            dx: -collision.delta_x*rate,
                            dy: -collision.delta_y*rate,
                        };
                        match resolutions.entry(entity) {
                            Entry::Occupied(mut entry) => entry.get_mut().push(resolution),
                            Entry::Vacant(entry) => {entry.insert(resolution);},
                        }
                    }

                }
            });

            physic_world.insert_movable(entity, &state.position, typ.group, &typ.shape);
        }

        for (entity,res) in resolutions {
            let state = states.get_mut(entity).unwrap();
            let typ = types.get(entity).unwrap();

            state.position[0] += res.dx;
            state.position[1] += res.dy;

            match typ.collision_behavior {
                CollisionBehavior::Bounce => {
                    let angle = state.velocity[1].atan2(state.velocity[0]) + PI;
                    state.velocity[0] = angle.cos();
                    state.velocity[1] = angle.sin();
                },
                CollisionBehavior::Stop => state.velocity = [0.,0.],
                CollisionBehavior::Back => {
                    state.velocity[0] = -state.velocity[0];
                    state.velocity[1] = -state.velocity[1];
                },
                CollisionBehavior::Persist => (),
            }
        }

        let fnv = BuildHasherDefault::<FnvHasher>::default();
        physic_world.movable_hashmap = HashMap::with_hasher(fnv);
        for (_,state,typ,entity) in (&dynamics, &mut states, &types, &entities).iter() {
            physic_world.insert_movable(entity, &state.position, typ.group, &typ.shape);
        }
    }
}

impl PhysicWorld {
    pub fn new() -> Self {
        let fnv0 = BuildHasherDefault::<FnvHasher>::default();
        let fnv1 = BuildHasherDefault::<FnvHasher>::default();

        let physic_world = PhysicWorld {
            unit: config.physic.unit,
            static_hashmap: HashMap::with_hasher(fnv0),
            movable_hashmap: HashMap::with_hasher(fnv1),
        };
        debug_assert_eq!(physic_world.cells_of_shape(&[0.5,0.5], &Shape::Square(0.5 + f32::EPSILON)).len(),1);
        debug_assert_eq!(physic_world.cells_of_shape(&[0.5,0.5], &Shape::Circle(0.5 + f32::EPSILON)).len(),1);

        physic_world
    }

    pub fn fill(&mut self, world: &specs::World) {
        let dynamics = world.read::<PhysicDynamic>();
        let statics = world.read::<PhysicStatic>();
        let states = world.read::<PhysicState>();
        let types = world.read::<PhysicType>();
        let entities = world.entities();

        self.static_hashmap.clear();
        self.movable_hashmap.clear();

        for (_,state,typ,entity) in (&dynamics, &states, &types, &entities).iter() {
            self.insert_movable(entity, &state.position, typ.group, &typ.shape);
        }
        for (_,state,typ,entity) in (&statics, &states, &types, &entities).iter() {
            self.insert_static(entity, &state.position, typ.group, &typ.shape);
        }
    }

    fn cells_of_shape(&self, pos: &[f32;2], shape: &Shape) -> Vec<[i32;2]> {
        let radius = match *shape {
            Shape::Circle(r) => r,
            Shape::Square(r) => r,
        };

        let min_x = ((pos[0]-radius+f32::EPSILON)/self.unit).floor() as i32;
        let max_x = ((pos[0]+radius-f32::EPSILON)/self.unit).ceil() as i32;
        let min_y = ((pos[1]-radius+f32::EPSILON)/self.unit).floor() as i32;
        let max_y = ((pos[1]+radius-f32::EPSILON)/self.unit).ceil() as i32;

        let mut cells = Vec::new();
        for x in min_x..max_x {
            for y in min_y..max_y {
                cells.push([x,y]);
            }
        }
        cells
    }

    pub fn apply_on_shape<F: FnMut(&specs::Entity,&Collision)>(&self, pos: &[f32;2], mask: u32, shape: &Shape, callback: &mut F) {
        let mut visited = HashSet::new();

        for cell in self.cells_of_shape(pos,shape) {
            self.apply_on_index(cell, mask, &mut |other_entity, other_pos, other_shape| {
                if visited.contains(other_entity) { return; }
                visited.insert(*other_entity);
                if let Some(collision) = shape_collide(pos,shape,other_pos,other_shape) {
                    callback(other_entity,&collision);
                }
            });
        }
    }

    fn apply_on_index<F: FnMut(&specs::Entity,&[f32;2],&Shape)>(&self, cell: [i32;2], mask: u32, callback: &mut F) {
        let empty_vec = vec!();
        let vec = self.movable_hashmap.get(&cell).unwrap_or(&empty_vec).iter()
            .chain(self.static_hashmap.get(&cell).unwrap_or(&empty_vec));

        for &(ref entity,ref pos, group, ref shape) in vec {
            if (group & mask) != 0 {
                callback(entity,pos,shape);
            }
        }
    }

    pub fn insert_static(&mut self, entity: specs::Entity, pos: &[f32;2], group: u32, shape: &Shape) {
        for cell in self.cells_of_shape(pos,shape) {
            self.static_hashmap.entry(cell).or_insert(Vec::new()).push((entity,pos.clone(),group,shape.clone()));
        }
    }

    // pub fn remove_static(&mut self, entity: specs::Entity, pos:&[f32;2], shape: &Shape) {
    //     for cell in self.cells_of_shape(pos,shape) {
    //         let vec = self.static_hashmap.get_mut(&cell).expect("remove static in an unexisting cell");
    //         let i = vec.iter().position(|&(e,_,_,_,_)| e == entity).expect("remove unfindable entity");
    //         vec.swap_remove(i);
    //     }
    // }

    fn insert_movable(&mut self, entity: specs::Entity, pos: &[f32;2], group: u32, shape: &Shape) {
        for cell in self.cells_of_shape(pos,shape) {
            self.movable_hashmap.entry(cell).or_insert(Vec::new()).push((entity,pos.clone(),group,shape.clone()));
        }
    }

    pub fn raycast<F: FnMut((specs::Entity,f32,f32)) -> bool>(&self, ray: &Ray, callback: &mut F) {
        use std::f32::consts::PI;
        use std::cmp::Ordering;
        use utils::minus_pi_pi;

        let angle = minus_pi_pi(ray.angle);

        let x0 = ray.origin[0];
        let y0 = ray.origin[1];
        let x1 = x0+ray.length*angle.cos();
        let y1 = y0+ray.length*angle.sin();
        let cells = grid_raycast(x0/self.unit, y0/self.unit, x1/self.unit, y1/self.unit);

        // equation ax + by + c = 0
        let (a,b,c) = if angle.abs() == PI || angle == 0. {
            (0.,1.,-y0)
        } else {
            let b = -1./angle.tan();
            (1.,b,-x0-b*y0)
        };

        let line_start = x0.min(x1);
        let line_end = x0.max(x1);

        let mut visited: HashSet<specs::Entity> = HashSet::new();

        for cell in cells {
            // abscisse of start and end the segment of
            // the line that is in the current square

            let segment_start = ((cell[0] as f32)*self.unit).max(line_start);
            let segment_end = (((cell[0]+1) as f32)*self.unit).min(line_end);

            let mut bodies: Vec<(specs::Entity,f32,f32)> = Vec::new();

            {
                let null_vec = vec!();
                let entities = self.movable_hashmap.get(&cell).unwrap_or(&null_vec).iter()
                    .chain(self.static_hashmap.get(&cell).unwrap_or(&null_vec).iter());

                for &(entity,ref pos,group,ref shape) in entities {
                    if (group & ray.mask) == 0 { continue; }
                    if visited.contains(&entity) { continue; }

                    let intersections = match *shape {
                        Shape::Circle(radius) => circle_raycast(pos[0],pos[1],radius,a,b,c),
                        Shape::Square(radius) => bounding_box_raycast(pos[0],pos[1],radius*2.,radius*2.,a,b,c),
                    };

                    if let Some((x_min,y_min,x_max,y_max)) = intersections {
                        // println!("intersection\nstart:{},end:{},min:{},max:{}",segment_start,segment_end,x_min,x_max);

                        // angle is between minus_pi and pi
                        if angle.abs() > PI/2. {
                            if segment_start <= x_max && x_min<= segment_end {
                                visited.insert(entity);
                                // println!("intersection in segment");
                                let max = ((x0-x_min).powi(2) + (y0-y_min).powi(2)).sqrt();
                                let mut min = ((x0-x_max).powi(2) + (y0-y_max).powi(2)).sqrt();
                                if x_max > segment_end {
                                    min = -min;
                                }
                                bodies.push((entity,min,max));
                            }
                        } else {
                            if segment_start <= x_max && x_min <= segment_end {
                                visited.insert(entity);
                                // println!("intersection in segment");
                                let mut min = ((x0-x_min).powi(2) + (y0-y_min).powi(2)).sqrt();
                                let max = ((x0-x_max).powi(2) + (y0-y_max).powi(2)).sqrt();
                                if x_min < segment_start {
                                    min = -min;
                                }
                                bodies.push((entity,min,max));
                            }
                        }
                    }
                }
            }

            bodies.sort_by(|&(_,min_a,_),&(_,min_b,_)| {
                if min_a > min_b {
                    Ordering::Greater
                } else if min_a == min_b {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            });

            for (entity,min,max) in bodies {
                if callback((entity,min,max)) {
                    return;
                }
            }
        }
    }
}

const MOVE: f32 = 7./8.;
const FACTOR: f32 = 8.*1.41421356237309504880;
fn shape_collide(a_pos: &[f32;2], a_shape: &Shape, b_pos: &[f32;2], b_shape: &Shape) -> Option<Collision> {
    match *a_shape {
        Shape::Circle(a_rad) => {
            match *b_shape {
                Shape::Circle(b_rad) => {
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
                },
                Shape::Square(b_rad) => {
                    let left = a_pos[0] < b_pos[0]-b_rad;
                    let right = a_pos[0] > b_pos[0]+b_rad;
                    let down = a_pos[1] < b_pos[1]-b_rad;
                    let up = a_pos[1] > b_pos[1]+b_rad;

                    let extern_horizontal = left || right;
                    let extern_vertical = up || down;

                    if extern_horizontal && extern_vertical {
                        let pos = if up && left {
                            [b_pos[0] - b_rad*MOVE, b_pos[1] + b_rad*MOVE]
                        } else if down && left {
                            [b_pos[0] - b_rad*MOVE, b_pos[1] - b_rad*MOVE]
                        } else if up && right {
                            [b_pos[0] + b_rad*MOVE, b_pos[1] + b_rad*MOVE]
                        } else {
                            [b_pos[0] + b_rad*MOVE, b_pos[1] - b_rad*MOVE]
                        };
                        let rad = b_rad/FACTOR;
                        shape_collide(a_pos,a_shape,&pos,&Shape::Circle(rad))
                    } else {
                        shape_collide(a_pos,&Shape::Square(a_rad),b_pos,b_shape)
                    }
                },
            }
        },
        Shape::Square(a_rad) => {
            match *b_shape {
                Shape::Circle(_) => {
                    shape_collide(b_pos,b_shape,a_pos,a_shape).map(|col| Collision {
                        delta_x: -col.delta_x,
                        delta_y: -col.delta_y,
                    })
                },
                Shape::Square(b_rad) => {
                    let a_min_x = a_pos[0] - a_rad;
                    let a_max_x = a_pos[0] + a_rad;
                    let a_min_y = a_pos[1] - a_rad;
                    let a_max_y = a_pos[1] + a_rad;
                    let b_min_x = b_pos[0] - b_rad;
                    let b_max_x = b_pos[0] + b_rad;
                    let b_min_y = b_pos[1] - b_rad;
                    let b_max_y = b_pos[1] + b_rad;

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
                },
            }
        },
    }
}

fn grid_raycast(x0: f32, y0: f32, x1: f32, y1: f32) -> Vec<[i32;2]> {
    if (x1-x0).abs() < (y1-y0).abs() {
        grid_raycast(y0,x0,y1,x1).iter().map(|s| [s[1],s[0]]).collect::<Vec<[i32;2]>>()
    } else if x0 == x1 {
        let x0_i32 = x0.floor() as i32;
        let y0_i32 = y0.floor() as i32;
        let y1_i32 = y1.floor() as i32;
        let mut vec = Vec::new();

        if y0 > y1 {
            for y in y1_i32..y0_i32+1 {
                vec.push([x0_i32,y]);
            }
            vec.reverse();
        } else {
            for y in y0_i32..y1_i32+1 {
                vec.push([x0_i32,y]);
            }
        }

        vec
    } else if x0 > x1 {
        let mut vec = grid_raycast(x1,y1,x0,y0);
        vec.reverse();
        vec
    } else {
        // x0 < x1
        //println!("x0:{},y0:{},x1:{},y1:{}",x0,y0,x1,y1);

        let x0_i32 = x0.floor() as i32;
        let y0_i32 = y0.floor() as i32;
        let x1_i32 = x1.floor() as i32;

        // equation y = ax + b
        let a = (y1 - y0)/(x1 - x0);
        let b = y0 -a*x0;
        //println!("a:{} b:{}",a,b);

        let delta_error = a.abs();

        let signum;
        if a > 0. {
            signum = 1;
        } else {
            signum = -1;
        }
        let mut error = if a > 0. {
            (a*x0.floor()+b)-y0.floor()
        } else {
            y0.ceil()-(a*x0.floor()+b)
        };

        //TODO cut some cells at the end
        //let mut error_end = if a > 0. {
        //    y1.ceil() - (a*x1.ceil()+b)
        //} else {
        //    (y1.floor() - (a*x1.ceil()+b))
        //};
        //println!("debut: error: {}",error);
        //println!("error end : {}", error_end);

        let mut vec = Vec::new();
        let mut y = y0_i32;

        for x in x0_i32..x1_i32+1 {
            vec.push([x,y]);
            error += delta_error;
            //println!("error: {}",error);
            while error >= 1.0 {
                y += signum;
                error -= 1.0;
                //println!("error -= 1.0: {}",error);
                vec.push([x,y]);
            }
        }
        //while error_end >= 0. {
        //    vec.pop();
        //    error_end -= 1.0;
        //}

        //println!("result: {:?}",vec);
        vec
    }
}

/// the coordinate of the intersections (if some) of a circle of center (x,y) and radius,
/// and the line of equation ax+by+c=0
fn circle_raycast(x: f32, y: f32, radius: f32, a: f32, b: f32, c: f32) -> Option<(f32,f32,f32,f32)> {
    if a == 0. && b == 0. {
        None
    } else if a == 0. {
        let y_ray = -c/b;
        if (y_ray - y).abs() < radius {
            let dx = (radius.powi(2) - (y_ray - y).powi(2)).sqrt();
            Some((x-dx,y_ray,x+dx,y_ray))
        } else {
            None
        }
    } else if b == 0. {
        let x_ray = -c/a;
        if (x_ray - x).abs() < radius {
            let dy = (radius.powi(2) - (x_ray - x).powi(2)).sqrt();
            Some((x_ray,y-dy,x_ray,y+dy))
        } else {
            None
        }
    } else {
        // the equation of intersection abscisse: d*x^2 + e*x + f = 0
        let d = 1. + (a/b).powi(2);
        let e = 2.*(-x + a/b*(c/b+y));
        let f = x.powi(2) + (c/b+y).powi(2) - radius.powi(2);

        let delta = e.powi(2) - 4.*d*f;

        if delta > 0. {
            let (x1,x2) = {
                let x1 = (-e - delta.sqrt())/(2.*d);
                let x2 = (-e + delta.sqrt())/(2.*d);
                if x1 > x2 {
                    (x2,x1)
                } else {
                    (x1,x2)
                }
            };
            let y1 = (-c-a*x1)/b;
            let y2 = (-c-a*x2)/b;

            Some((x1,y1,x2,y2))
        } else {
            None
        }
    }
}

#[test]
fn circle_raycast_test() {
    // for a == 0
    assert_eq!(Some((-1.,3.,3.,3.)),circle_raycast(1.,3.,2.,0.,-1.,3.));

    // for b == 0
    assert_eq!(Some((3.,0.,3.,2.)),circle_raycast(3.,1.,1.,-1.,0.,3.));

    // for b != 0 && a != 0
    assert_eq!(Some((-0.99999994,-0.99999994,0.99999994,0.99999994)),circle_raycast(0.,0.,2f32.sqrt(),1.,-1.,0.));
}

/// the coordinate of the intersections (if some) of a rectangle of center (x,y) width and height,
/// and the line of equation ax+by+c=0
fn bounding_box_raycast(x: f32, y: f32, width: f32, height: f32, a: f32, b: f32, c: f32) -> Option<(f32,f32,f32,f32)> {
    if a == 0. && b == 0. {
        None
    } else if a == 0. {
        let y_proj = -c/b;
        if y - height/2. <= y_proj && y_proj <= y + height/2. {
            Some((x-width/2.,y_proj,x+width/2.,y_proj))
        } else {
            None
        }
    } else if b == 0. {
        let x_proj = -c/a;
        if x - width/2. <= x_proj && x_proj <= x + width/2. {
            Some((x_proj,y-height/2.,x_proj,y+height/2.))
        } else {
            None
        }
    } else {
        //println!("x:{}, y:{}, width:{}, height:{}, a:{}, b:{}, c:{}",x,y,width,height,a,b,c);
        // the ordonate of the point that is on the line(a,b) and the horizontal line that cut (x,y)
        let y_proj = -(a*x + c)/b;
        // the abscisse of the point that is on the line(a,b) and the vertical line that cut (x,y)
        let x_proj = -(b*y+c)/a;

        //println!("proj: {:?} | {:?}",x_proj,y_proj);
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

        let dx = -height/2. * b/a;
        //println!("dx: {:?}",dx);
        let x_i = x_proj + dx;
        let y_i = y + height/2.;
        let x_k = x_proj - dx;
        let y_k = y - height/2.;
        //println!("i: {:?} | {:?}",x_i,y_i);
        //println!("k: {:?} | {:?}",x_k,y_k);

        let dy = -width/2. * a/b;
        //println!("dy: {:?}",dy);
        let x_j = x + width/2.;
        let y_j = y_proj + dy;
        let x_l = x - width/2.;
        let y_l = y_proj - dy;
        //println!("j: {:?} | {:?}",x_j,y_j);
        //println!("l: {:?} | {:?}",x_l,y_l);


        let cond_i = x-width/2. < x_i && x_i < x+width/2.;
        let cond_k = x-width/2. < x_k && x_k < x+width/2.;
        let cond_j = y-width/2. < y_j && y_j < y+width/2.;
        let cond_l = y-width/2. < y_l && y_l < y+width/2.;

        if cond_i && cond_k {
            if x_i < x_k {
                Some((x_i,y_i,x_k,y_k))
            } else {
                Some((x_k,y_k,x_i,y_i))
            }
        } else if cond_i && cond_j {
            if x_i < x_j {
                Some((x_i,y_i,x_j,y_j))
            } else {
                Some((x_j,y_j,x_i,y_i))
            }
        } else if cond_i && cond_l {
            if x_i < x_l {
                Some((x_i,y_i,x_l,y_l))
            } else {
                Some((x_l,y_l,x_i,y_i))
            }
        } else if cond_j && cond_k {
            if x_j < x_k {
                Some((x_j,y_j,x_k,y_k))
            } else {
                Some((x_k,y_k,x_j,y_j))
            }
        } else if cond_j && cond_l {
            if x_j < x_l {
                Some((x_j,y_j,x_l,y_l))
            } else {
                Some((x_l,y_l,x_j,y_j))
            }
        } else if cond_k && cond_l {
            if x_k < x_l {
                Some((x_k,y_k,x_l,y_l))
            } else {
                Some((x_l,y_l,x_k,y_k))
            }
        } else {
            None
        }
    }
}

#[test]
fn test_bounding_box_raycast() {
    // for a == 0
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 0., 0., -1.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 0., 1., -1.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 0., -1./0.5, -1.));
    assert_eq!(Some((-4.,-1.,2.,-1.)),bounding_box_raycast( -1., -2., 6., 2., 0., -1./1., -1.));
    assert_eq!(Some((-4.,-2.,2.,-2.)),bounding_box_raycast( -1., -2., 6., 2., 0., -1./2., -1.));
    assert_eq!(Some((-4.,-3.,2.,-3.)),bounding_box_raycast( -1., -2., 6., 2., 0., -1./3., -1.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 0., -1./3.5, -1.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 0., -1./4., -1.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 0., -1./4.5, -1.));

    // for b == 0
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., -1./4.5, 0., -1.));
    assert_eq!(Some((-4.,-3.,-4.,-1.)),bounding_box_raycast( -1., -2., 6., 2., -1./4., 0., -1.));
    assert_eq!(Some((0.,-3.,0.,-1.)),bounding_box_raycast( -1., -2., 6., 2., 1., 0., 0.));
    assert_eq!(Some((2.,-3.,2.,-1.)),bounding_box_raycast( -1., -2., 6., 2., 1./2., 0., -1.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 1./2.5, 0., -1.));

    // for b != 0 && a != 0
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., -1., -1., 9.));
    assert_eq!(None,bounding_box_raycast( -1., -2., 6., 2., 1., 1., 7.));
    assert_eq!(Some((-4.,-2.,-3.,-3.)),bounding_box_raycast( -1., -2., 6., 2., 1., 1., 6.));

    assert_eq!(Some((-4.,-1.96,2.,-2.02)),bounding_box_raycast( -1., -2., 6., 2., 0.01, 1., 2.));
}

