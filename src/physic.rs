use specs;

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

