use viewport::Viewport;
use opengl_graphics::GlGraphics;
use world::Camera;
use world::body::BodyTrait;
use super::Localisable;

// enum used when classifying an object in the sons of 
// a node.
enum Quadrant {
    Upleft,
    Upright,
    Downleft,
    Downright,
    Nil,
}

// quadrant! take 3 parametre:
// * the x coordinate for the vertical axe
// * the y coordinate for the horizontal axe 
// * a Localisable object
macro_rules! quadrant {
    ($x: ident, $y: ident, $obj: ident) => (
        {
            if $obj.up($y) {
                if $obj.right($x) {
                    Quadrant::Upright
                } else if $obj.left($x) {
                    Quadrant::Upleft
                } else {
                    Quadrant::Nil
                }
            } else if $obj.down($y) {
                if $obj.right($x) {
                    Quadrant::Downright
                } else if $obj.left($x) {
                    Quadrant::Downleft
                } else {
                    Quadrant::Nil
                }
            } else {
                Quadrant::Nil
            }
        }
    )
}

/// OwnerQuadtree represent a node of the quadtree,
///
/// * level is the level where it cannot split anymore,
/// * max_object is the number of object necessar to split the node,
/// * level is the level of the node,
/// * objects contains all the objects of the node if the node isn't splited
/// otherwise it contains all the objects that cannot be stored in sons' node.
/// * is the bounding box of the node
/// * nodes are its sons if they are.
/// * x and y are downleft coordinate
///
/// you must be careful with lifetime: for performance purpose the quadtree 
/// borrow objects and store this borrow. that's why objects must live longer
/// than quadtree. And those objects will be mutable again when the quadtree is
/// gone
pub struct OwnerQuadtree {
    max_object: usize,
    level: usize,
    objects: Vec<Box<BodyTrait>>,
    x: f64, //downleft
    y: f64, //downleft
    width: f64,
    height: f64,
    nodes: OwnerQuadtreeBatch,
}

// OwnerQuadtreeBatch represent the sons of a node,
// sons can be none -> Nil,
// otherwise they're 4 -> Cons(..,..,..,..)
pub enum OwnerQuadtreeBatch {
    Cons {
        upleft: Box<OwnerQuadtree>,
        upright: Box<OwnerQuadtree>,
        downright: Box<OwnerQuadtree>,
        downleft: Box<OwnerQuadtree>,
    },
    Nil,
}
impl OwnerQuadtree {
    /// create a new OwnerQuadtree 
    pub fn new(x: f64, y: f64, width: f64, height: f64, max_obj: usize, max_lvl: usize) -> OwnerQuadtree {

        OwnerQuadtree {
            level: max_lvl,
            max_object: max_obj,

            objects: vec![],

            x: x,
            y: y,

            width: width,
            height: height,

            nodes: OwnerQuadtreeBatch::Nil,
        }
    }

    fn split(&mut self) {
        let max_lvl = self.level;
        let max_obj = self.max_object;

        let sub_width = self.width/2.;
        let sub_height = self.height/2.;
        let x = self.x;
        let y = self.y;

        let mut downleft = Box::new(OwnerQuadtree::new(x, y, sub_width, sub_height, max_obj, max_lvl-1));
        let mut downright = Box::new(OwnerQuadtree::new(x+sub_width, y, sub_width, sub_height, max_obj, max_lvl-1));
        let mut upright = Box::new(OwnerQuadtree::new(x+sub_width, y+sub_height, sub_width, sub_height, max_obj, max_lvl-1));
        let mut upleft = Box::new(OwnerQuadtree::new(x, y+sub_height, sub_width, sub_height, max_obj, max_lvl-1));

        let center_x = self.x + self.width/2.;
        let center_y = self.y + self.height/2.;

        let mut objects = Vec::<Box<BodyTrait>>::new();
        objects.append(&mut self.objects);

        while let Some(obj) = objects.pop() {

            match quadrant!(center_x,center_y,obj) {
                Quadrant::Upright => { upright.insert(obj); },
                Quadrant::Upleft => { upleft.insert(obj); },
                Quadrant::Downright => { downright.insert(obj); },
                Quadrant::Downleft => { downleft.insert(obj); },
                Quadrant::Nil => { self.objects.push(obj); },
            }

        }

        self.nodes = OwnerQuadtreeBatch::Cons {
            downleft: downleft,
            downright: downright,
            upright: upright,
            upleft: upleft,
        };
    }

    pub fn insert(&mut self, obj: Box<BodyTrait>) {
        if self.level == 0 {
            self.objects.push(obj);
            return;
        }
        if let OwnerQuadtreeBatch::Nil = self.nodes {
            if self.max_object == self.objects.len() {
                self.split();
            } else {
                self.objects.push(obj);
                return;
            }
        }

        if let OwnerQuadtreeBatch::Cons { ref mut upleft, ref mut upright, ref mut downright, ref mut downleft } = self.nodes {
            let center_x = self.x + self.width/2.;
            let center_y = self.y + self.height/2.;

            match quadrant!(center_x,center_y,obj) {
                Quadrant::Upright => { upright.insert(obj); },
                Quadrant::Upleft => { upleft.insert(obj); },
                Quadrant::Downright => {downright.insert(obj); },
                Quadrant::Downleft => { downleft.insert(obj); },
                Quadrant::Nil => { self.objects.push(obj); },
            }
        }
    }

    #[allow(dead_code)]
    pub fn render_debug(&self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
        use graphics::Transformed;
        use graphics::line::{ 
            Line as LineDrawer, 
            Shape as LineShape,
        };
        use graphics::types::Line;
        use graphics::default_draw_state;

        if let OwnerQuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {
            #[allow(dead_code)]
            const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.5]; 

            let line_drawer = LineDrawer {
                color: GREEN,
                radius: 1.,
                shape: LineShape::Round,
            };

            let mut lines: Vec<Line> = vec![];

            lines.push([
                       self.x,
                       self.y+self.height/2.,
                       self.x+self.width,
                       self.y+self.height/2.]);

            lines.push([
                       self.x+self.width/2.,
                       self.y,
                       self.x+self.width/2.,
                       self.y+self.height]);

            gl.draw(*viewport, |context, gl| {
                let transform = camera.trans(context.transform);

                for line in lines {
                    line_drawer.draw(line, default_draw_state(), transform, gl);
                }
            });

            upleft.render_debug(viewport,camera,gl);
            downleft.render_debug(viewport,camera,gl);
            upright.render_debug(viewport,camera,gl);
            downright.render_debug(viewport,camera,gl);
        }
    }

    pub fn render(&self, viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) {
        for obj in &self.objects {
            obj.render(viewport,camera,gl);
        }
        if let OwnerQuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {
            upleft.render(viewport,camera,gl);
            downleft.render(viewport,camera,gl);
            upright.render(viewport,camera,gl);
            downright.render(viewport,camera,gl);
        }
    }

    pub fn apply<F: FnMut(&BodyTrait)>(&self, callback: &mut F) {
        for obj in &self.objects {
            callback(&**obj);
        }
        if let OwnerQuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {
            upright.apply(callback);
            upleft.apply(callback);
            downright.apply(callback);
            downleft.apply(callback);
        }
    }

    pub fn apply_locally<F: FnMut(&BodyTrait)>(&self, loc: &Localisable, callback: &mut F) {
        for obj in &self.objects {
            callback(&**obj);
        }

        if let OwnerQuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {
            let center_x = self.x + self.width/2.;
            let center_y = self.y + self.height/2.;

            match quadrant!(center_x,center_y,loc) {
                Quadrant::Upright => upright.apply_locally(loc,callback),
                Quadrant::Upleft => upleft.apply_locally(loc,callback),
                Quadrant::Downright => downright.apply_locally(loc,callback),
                Quadrant::Downleft => downleft.apply_locally(loc,callback),
                Quadrant::Nil => {
                    upright.apply(callback);
                    upleft.apply(callback);
                    downright.apply(callback);
                    downleft.apply(callback);
                },
            }
        }
    }
}
