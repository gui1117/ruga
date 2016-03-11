use super::{
    BodyTrait,
    CollisionBehavior,
    BodyType,
    PhysicType,
};
use frame_manager::FrameManager;

pub struct Body {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub id: usize,
    pub weight: f64,
    pub velocity: f64,
    pub angle: f64,
    pub mask: u32,
    pub group: u32,
    pub collision_behavior: CollisionBehavior,
    pub body_type: BodyType,
    pub physic_type: PhysicType,
}

impl Body {
    pub fn update(&mut self, dt: f64) {
        if self.velocity != 0. {
            self.x += dt*self.velocity()*self.angle().cos();
            self.y += dt*self.velocity()*self.angle().sin();
        }
    }

    pub fn render(&self, color: [f64;4], frame_manager: &mut FrameManager) {
        frame_manager.draw_square(color,self.x,self.y,self.width,self.height);
    }
}

impl BodyTrait for Body {
    fn id(&self) -> usize {
        self.id
    }

    fn physic_type(&self) -> PhysicType {
        self.physic_type.clone()
    }

    fn dead(&self) -> bool {
        false
    }

    fn body_type(&self) -> BodyType {
        self.body_type.clone()
    }

    fn damage(&mut self, _: f64) {
    }

    fn width(&self) -> f64 {
        self.width
    }

    fn height(&self) -> f64 {
        self.height
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    fn weight(&self) -> f64 {
        self.weight
    }

    fn velocity(&self) -> f64 {
        self.velocity
    }

    fn set_velocity(&mut self, v: f64) {
        self.velocity = v;
    }

    fn angle(&self) -> f64 {
        self.angle
    }

    fn set_angle(&mut self, a: f64) {
        self.angle = a;
    }

    fn mask(&self) -> u32 {
        self.mask
    }

    fn group(&self) -> u32 {
        self.group
    }

    fn collision_behavior(&self) -> CollisionBehavior {
        self.collision_behavior.clone()
    }

    fn on_collision(&mut self, _other: &mut BodyTrait) {
    }
}

#[test]
fn test_trait_in_circle() {
    let a = Body {
        x: 1.,
        y: 2.,
        width: 2.,
        height: 2.,
        id: 1,
        weight: 0.,
        velocity: 0.,
        angle: 0.,
        mask: 0,
        group: 0,
        collision_behavior: CollisionBehavior::Stop,
        body_type: BodyType::Wall,
    };
    assert_eq!(true,a.in_circle([3.,3.],1.));
    assert_eq!(false,a.in_circle([3.,3.],0.9));
    assert_eq!(true,a.in_circle([3.,2.],1.1));
    assert_eq!(true,a.in_circle([1.,2.],0.1));
    assert_eq!(true,a.in_circle([1.,2.],5.1));

    let b = Body {
        x: 0.,
        y: 0.,
        width: 20.,
        height: 2.,
        id: 1,
        weight: 0.,
        velocity: 0.,
        angle: 0.,
        mask: 0,
        group: 0,
        collision_behavior: CollisionBehavior::Stop,
        body_type: BodyType::Wall,
    };
    assert_eq!(true,b.in_circle([0.,5.],5.));
    assert_eq!(true,b.in_circle([5.,0.],10.));
}

