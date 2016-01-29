use viewport::Viewport;
use opengl_graphics::GlGraphics;
use world::Camera;
use super::{ 
    BodyTrait,
    CollisionBehavior,
    BodyType,
};
use std::cell::RefCell;

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
    pub body_type: BodyType,
}

/// model for delegate
///    delegate!{
///        body:
///            id() -> usize,
///            body_type() -> BodyType,
///            damage(d: f64) -> (),
///            width2() -> f64,
///            height2() -> f64,
///            x() -> f64,
///            mut set_x(x: f64) -> (),
///            y() -> f64,
///            mut set_y(y: f64) -> (),
///            weight() -> f64,
///            velocity() -> f64,
///            mut set_velocity(v: f64) -> (),
///            angle() -> f64,
///            mut set_angle(a: f64) -> (),
///            mask() -> u32,
///            group() -> u32,
///            collision_behavior() -> CollisionBehavior,
///            render(viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) -> (),
///            render_debug(lines: &mut Vec<[f64;4]>) -> (),
///            on_collision(other: &BodyTrait) -> (),
///            mut update(dt: f64) -> (),
///    }

impl Body {
    pub fn id(&self) -> usize {
        self.id
    }
    
    pub fn body_type(&self) -> BodyType {
        self.body_type.clone()
    }

    pub fn damage(&self, _: f64) {
    }

    pub fn width2(&self) -> f64 {
        self.width2
    }

    pub fn height2(&self) -> f64 {
        self.height2
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

    pub fn velocity(&self) -> f64 {
        self.velocity
    }

    pub fn set_velocity(&mut self, v: f64) {
        self.velocity = v;
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn set_angle(&mut self, a: f64) {
        self.angle = a;
    }

    pub fn mask(&self) -> u32 {
        self.mask
    }

    pub fn group(&self) -> u32 {
        self.group
    }

    pub fn update(&mut self, dt: f64) {
        if self.velocity != 0. {
            self.x += dt*self.velocity()*self.angle().cos();
            self.y += dt*self.velocity()*self.angle().sin();
        }
    }

    pub fn collision_behavior(&self) -> CollisionBehavior {
        self.collision_behavior.clone()
    }

    pub fn render_debug(&self, lines: &mut Vec<[f64;4]>) {
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
    }

    pub fn render(&self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
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

    pub fn on_collision(&self, _other: &BodyTrait) {
    }
}

impl BodyTrait for RefCell<Body> {
    fn id(&self) -> usize {
        self.borrow().id
    }
    
    fn body_type(&self) -> BodyType {
        self.borrow().body_type.clone()
    }

    fn damage(&self, _: f64) {
    }

    fn width2(&self) -> f64 {
        self.borrow().width2
    }

    fn height2(&self) -> f64 {
        self.borrow().height2
    }

    fn x(&self) -> f64 {
        self.borrow().x
    }

    fn set_x(&self, x: f64) {
        self.borrow_mut().x = x;
    }

    fn y(&self) -> f64 {
        self.borrow().y
    }

    fn set_y(&self, y: f64) {
        self.borrow_mut().y = y;
    }

    fn weight(&self) -> f64 {
        self.borrow().weight
    }

    fn velocity(&self) -> f64 {
        self.borrow().velocity
    }

    fn set_velocity(&self, v: f64) {
        self.borrow_mut().velocity = v;
    }

    fn angle(&self) -> f64 {
        self.borrow().angle
    }

    fn set_angle(&self, a: f64) {
        self.borrow_mut().angle = a;
    }

    fn mask(&self) -> u32 {
        self.borrow().mask
    }

    fn group(&self) -> u32 {
        self.borrow().group
    }

    fn update(&self, dt: f64) {
        self.borrow_mut().update(dt);
    }

    fn collision_behavior(&self) -> CollisionBehavior {
        self.borrow().collision_behavior.clone()
    }

    fn render_debug(&self, lines: &mut Vec<[f64;4]>) {
        self.borrow().render_debug(lines);
    }

    fn render(&self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
        self.borrow().render(viewport,camera,gl);
    }

    fn on_collision(&self, other: &BodyTrait) {
        self.borrow().on_collision(other);
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
