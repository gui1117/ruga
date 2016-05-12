use specs;
use UpdateContext;
use specs::Join;
use std::collections::HashMap;
use std::collections::HashSet;

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
pub struct PhysicState {
    pub position: [f32;2],
    pub velocity: [f32;2],
    pub acceleration: [f32;2],
}
impl PhysicState {
    pub fn new() -> Self {
        PhysicState{
            position: [0.,0.],
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
}
impl PhysicType {
    pub fn new(shape: Shape, collision: CollisionBehavior, velocity: f32, time_to_reach_v_max: f32, weight: f32) -> Self {
        let rate: f32 = 0.9;
        let damping = -weight * rate.ln() / time_to_reach_v_max;
        let force = velocity * damping;
        PhysicType {
            shape: shape,
            collision_behavior: collision,
            weight: weight,
            damping: damping,
            force: force,
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
}
impl specs::Component for PhysicForce {
    type Storage = specs::VecStorage<Self>;
}

// pub struct PhysicSystem {
// }
// impl specs::System<UpdateContext> for PhysicSystem {
//     fn run(&mut self, run_arg: specs::RunArg, context: UpdateContext) {
//         let entities = run_arg.fetch(|world|
//             world.entities()
//         );

//             // for (a_entity, a_type, a_state) in (&entities, &types, &states).iter() {
//             //     for (b_entity, b_type, b_state) in (&entities, &types, &states).iter() {
//             //         if a_entity.get_id() > b_entity.get_id() {
//             //             // if collision store resolution
//             //         }
//             //     }
//             // }

//             //for entity {
//             //  resolve collision
//             //  store in a spatial_hashing
//             //}
//         // });
//     }
// }

// pub struct Location {
//     pub center: [f32;2],
//     pub width: f32,
//     pub height: f32,
// }

// impl Location {
//     pub fn from_physic(pos: &[f32;2], shape: &Shape) -> Self {
//         let radius = match *shape {
//             Shape::Circle(r) => r,
//             Shape::Square(r) => r,
//         };
//         Location {
//             center: pos.clone(),
//             width: radius,
//             height: radius,
//         }
//     }
//     fn cells(&self,unit: f32) -> Vec<[i32;2]> {
//         let min_x = ((self.center[0]-self.width/2.)/unit).floor() as i32;
//         let max_x = ((self.center[0]+self.width/2.)/unit).ceil() as i32;
//         let min_y = ((self.center[1]-self.height/2.)/unit).floor() as i32;
//         let max_y = ((self.center[1]+self.height/2.)/unit).ceil() as i32;

//         (min_x..max_x)
//             .zip(min_y..max_y)
//             .map(|(x,y)| [x,y])
//             .collect()
//     }
// }

pub struct Ray {
    pub origin: [f32;2],
    pub angle: f32,
    pub length: f32,
}

pub struct PhysicWorld {
    unit: f32,
    static_hashmap: HashMap<[i32;2],Vec<(specs::Index,[f32;2],Shape)>>,
    dynamic_hashmap: HashMap<[i32;2],Vec<(specs::Index,[f32;2],Shape)>>,
}

struct Collision {
}

fn shape_collide(a_pos: &[f32;2], a_shape: &Shape, b_pos: &[f32;2], b_shape: &Shape) -> Option<Collision> {
    unimplemented!();
}

impl PhysicWorld {
    fn cells_of_shape(&self, pos: &[f32;2], shape: &Shape) -> Vec<[i32;2]> {
        let radius = match *shape {
            Shape::Circle(r) => r,
            Shape::Square(r) => r,
        };

        let min_x = ((pos[0]-radius/2.)/self.unit).floor() as i32;
        let max_x = ((pos[0]+radius/2.)/self.unit).ceil() as i32;
        let min_y = ((pos[1]-radius/2.)/self.unit).floor() as i32;
        let max_y = ((pos[1]+radius/2.)/self.unit).ceil() as i32;

        (min_x..max_x)
            .zip(min_y..max_y)
            .map(|(x,y)| [x,y])
            .collect()
    }

    pub fn apply_on_shape<F: FnMut(&specs::Index,&[f32;2],&Shape,&Collision)>(&self, pos: &[f32;2], shape: &Shape, callback: &mut F) {
        let mut visited = HashSet::new();

        for cell in self.cells_of_shape(pos,shape) {
            self.apply_on_index(cell, &mut |other_id, other_pos, other_shape| {
                if !visited.contains(other_id) { return; }
                visited.insert(*other_id);
                if let Some(collision) = shape_collide(pos,shape,other_pos,other_shape) {
                    callback(other_id,other_pos,other_shape,&collision);
                }
            });
        }
    }

    pub fn update(&mut self, dt: f32, world: &specs::World) {
        use specs::Join;

        let mut states = world.write::<PhysicState>();
        let forces= world.read::<PhysicForce>();
        let types = world.read::<PhysicType>();
        let entities = world.entities();

        self.dynamic_hashmap = HashMap::new();

        for (state,force,typ,entity) in (&mut states, &forces, &types, &entities).iter() {
            // acceleration
            // velocity
            // position

            self.apply_on_shape(&state.position, &typ.shape, &mut |other_id,_,_,collision| {
                // resolve collision
            });

            self.insert_dynamic(entity.get_id(), &state.position, &typ.shape);
        }
    }

    pub fn apply_on_index<F: FnMut(&specs::Index,&[f32;2],&Shape)>(&self, cell: [i32;2], callback: &mut F) {
        if let Some(vec) = self.dynamic_hashmap.get(&cell) {
            for &(ref id,ref pos,ref shape) in vec {
                callback(id,pos,shape);
            }
        }
        if let Some(vec) = self.static_hashmap.get(&cell) {
            for &(ref id,ref pos,ref shape) in vec {
                callback(id,pos,shape);
            }
        }
    }

    pub fn insert_static(&mut self, index: specs::Index, pos: &[f32;2], shape: &Shape) {
        for cell in self.cells_of_shape(pos,shape) {
            self.static_hashmap.entry(cell).or_insert(Vec::new()).push((index,pos.clone(),shape.clone()));
        }
    }

    pub fn insert_dynamic(&mut self, index: specs::Index, pos: &[f32;2], shape: &Shape) {
        for cell in self.cells_of_shape(pos,shape) {
            self.dynamic_hashmap.entry(cell).or_insert(Vec::new()).push((index,pos.clone(),shape.clone()));
        }
    }

    pub fn raycast<F: FnMut((specs::Index,f32,f32)) -> bool>(&self, ray: &Ray, callback: &mut F) {
        use std::f32::consts::PI;
        use std::cmp::Ordering;
        use utils::{minus_pi_pi, grid_raycast, bounding_box_raycast, circle_raycast};

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

        let mut visited: HashSet<specs::Index> = HashSet::new();

        for cell in cells {
            // abscisse of start and end the segment of
            // the line that is in the current square

            let segment_start = ((cell[0] as f32)*self.unit).max(line_start);
            let segment_end = (((cell[0]+1) as f32)*self.unit).min(line_end);

            let mut bodies: Vec<(specs::Index,f32,f32)> = Vec::new();

            {
                let null_vec = vec!();
                let entities = self.dynamic_hashmap.get(&cell).unwrap_or(&null_vec).iter()
                    .chain(self.static_hashmap.get(&cell).unwrap_or(&null_vec).iter());

                for &(entity,ref pos,ref shape) in entities {
                    if visited.contains(&entity) { continue; }

                    // let intersections = entity.borrow().body().raycast(a,b,c);
                    let intersections = match *shape {
                        Shape::Circle(radius) => circle_raycast(pos[0],pos[1],radius,a,b,c),
                        Shape::Square(radius) => bounding_box_raycast(pos[0],pos[1],radius,radius,a,b,c),
                    };

                    if let Some((x_min,y_min,x_max,y_max)) = intersections {
                        // println!("intersection\nstart:{},end:{},min:{},max:{}",segment_start,segment_end,x_min,x_max);

                        // angle is between minus_pi and pi
                        if angle.abs() > PI/2. {
                            if segment_start <= x_max && x_min <= segment_end {
                                visited.insert(entity);
                                //println!("intersection in segment");
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
                                //println!("intersection in segment");
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

