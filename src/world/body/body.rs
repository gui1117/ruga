use viewport::Viewport;
use opengl_graphics::GlGraphics;
use world::Camera;
use super::{ 
    BodyTrait,
    CollisionBehavior,
};

pub struct Body {
    pub x: f64,
    pub y: f64,
    pub width2: f64,
    pub height2: f64,
    pub id: usize,
    pub weight: f64,
    pub velocity: f64,
    pub angle: f64,
    pub mask: u32,
    pub group: u32,
    pub collision_behavior: CollisionBehavior,
}

/// model for delegate
/// delegate!{
///     body:
///        id() -> usize,
///        width2() -> f64,
///        height2() -> f64,
///        x() -> f64,
///        mut set_x(x: f64) -> (),
///        y() -> f64,
///        mut set_y(y: f64) -> (),
///        weight() -> f64,
///        velocity() -> f64,
///        mut set_velocity(v: f64) -> (),
///        angle() -> f64,
///        mut set_angle(a: f64) -> (),
///        mask() -> u32,
///        group() -> u32,
///        mut update(dt: f64) -> (),
///        collision_behavior() -> CollisionBehavior,
///        render(viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) -> (),
///        mut on_collision(other: &BodyTrait) -> (),
/// }

impl BodyTrait for Body {
    fn id(&self) -> usize {
        self.id
    }

    fn width2(&self) -> f64 {
        self.width2
    }

    fn height2(&self) -> f64 {
        self.height2
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

    fn update(&mut self, dt: f64) {
        if self.velocity != 0. {
            self.x += dt*self.velocity()*self.angle().cos();
            self.y += dt*self.velocity()*self.angle().sin();
        }
    }

    fn collision_behavior(&self) -> CollisionBehavior {
        self.collision_behavior.clone()
    }

    fn render(&self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
        use graphics::Transformed;
        use graphics::line::{ 
            Line as LineDrawer, 
            Shape as LineShape,
        };
        use graphics::types::Line;
        use graphics::default_draw_state;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.5]; 

        let line_drawer = LineDrawer {
            color: RED,
            radius: 1.,
            shape: LineShape::Round,
        };

        let mut lines: Vec<Line> = Vec::with_capacity(4);
        lines.push([
                   self.x-self.width2,self.y-self.height2,
                   self.x-self.width2,self.y+self.height2,
        ]);
        lines.push([
                   self.x-self.width2,self.y+self.height2,
                   self.x+self.width2,self.y+self.height2,
        ]);
        lines.push([
                   self.x+self.width2,self.y+self.height2,
                   self.x+self.width2,self.y-self.height2,
        ]);
        lines.push([
                   self.x+self.width2,self.y-self.height2,
                   self.x-self.width2,self.y-self.height2,
        ]);

        gl.draw(*viewport, |context, gl| {
            let transform = camera.trans(context.transform);

            for line in lines {
                line_drawer.draw(line, default_draw_state(), transform, gl);
            }
        });
    }

    fn on_collision(&mut self, _other: &BodyTrait) {
    }
}

//impl BodyTrait for Rc<Body> {
//    fn id(&self) -> usize {
//        (self as &Body).id()
//    }
//
//    fn width2(&self) -> f64 {
//        (self as &Body).width2()
//    }
//
//    fn height2(&self) -> f64 {
//        (self as &Body).height2()
//    }
//
//    fn x(&self) -> f64 {
//        (self as &Body).x()
//    }
//
//    fn set_x(&mut self, x: f64) {
//    }
//
//    fn y(&self) -> f64 {
//        (self as &Body).y()
//    }
//
//    fn set_y(&mut self, y: f64) {
//    }
//
//    fn weight(&self) -> f64 {
//        (self as &Body).weight()
//    }
//
//    fn velocity(&self) -> f64 {
//        (self as &Body).velocity()
//    }
//
//    fn set_velocity(&mut self, v: f64) {
//    }
//
//    fn angle(&self) -> f64 {
//        (self as &Body).angle()
//    }
//
//    fn set_angle(&mut self, a: f64) {
//    }
//
//    fn mask(&self) -> u32 {
//        (self as &Body).mask()
//    }
//
//    fn group(&self) -> u32 {
//        (self as &Body).group()
//    }
//
//    fn update(&mut self, dt: f64) {
//    }
//
//    fn collision_behavior(&self) -> CollisionBehavior {
//        (self as &Body).collision_behavior()
//    }
//
//    fn render(&self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
//        (self as &Body).render(viewport,camera,gl)
//    }
//
//    fn on_collision(&mut self, _other: &BodyTrait) {
//    }
//}
//
//impl BodyTrait for Rc<RefCell<Body>> {
//    fn id(&self) -> usize {
//        self.borrow().id()
//    }
//
//    fn width2(&self) -> f64 {
//        self.borrow().width2()
//    }
//
//    fn height2(&self) -> f64 {
//        self.borrow().height2()
//    }
//
//    fn x(&self) -> f64 {
//        self.borrow().x()
//    }
//
//    fn set_x(&mut self, x: f64) {
//        self.borrow_mut().set_x(x);
//    }
//
//    fn y(&self) -> f64 {
//        self.borrow().y()
//    }
//
//    fn set_y(&mut self, y: f64) {
//        self.borrow_mut().set_y(y);
//    }
//
//    fn weight(&self) -> f64 {
//        self.borrow().weight()
//    }
//
//    fn velocity(&self) -> f64 {
//        self.borrow().velocity()
//    }
//
//    fn set_velocity(&mut self, v: f64) {
//        self.borrow_mut().set_velocity(v);
//    }
//
//    fn angle(&self) -> f64 {
//        self.borrow().angle()
//    }
//
//    fn set_angle(&mut self, a: f64) {
//        self.borrow_mut().set_angle(a);
//    }
//
//    fn mask(&self) -> u32 {
//        self.borrow().mask()
//    }
//
//    fn group(&self) -> u32 {
//        self.borrow().group()
//    }
//
//    fn update(&mut self, dt: f64) {
//        self.borrow_mut().update(dt);
//    }
//
//    fn collision_behavior(&self) -> CollisionBehavior {
//        self.borrow().collision_behavior()
//    }
//
//    fn render(&self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
//        self.borrow().render(viewport,camera,gl);
//    }
//
//    fn on_collision(&mut self, other: &BodyTrait) {
//        self.borrow_mut().on_collision(other);
//    }
//}
//
