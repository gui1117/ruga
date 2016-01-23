use viewport::Viewport;
use opengl_graphics::GlGraphics;

use world::{ 
    Camera,
    BodyTrait,
};
use world::quadtree::{
    FixedQuadtree,
    FixedQuadtreeBatch,
};

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

/// BorrowerQuadtree represent a node of the quadtree,
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
pub struct BorrowerQuadtree<'l, T: 'l + BodyTrait> {
    max_object: usize,
    level: usize,
    objects: Vec<&'l T>,
    x: f64, //downleft
    y: f64, //downleft
    width: f64,
    height: f64,
    nodes: BorrowerQuadtreeBatch<'l, T>,
}

// BorrowerQuadtreeBatch represent the sons of a node,
// sons can be none -> Nil,
// otherwise they're 4 -> Cons(..,..,..,..)
pub enum BorrowerQuadtreeBatch<'l, T: 'l + BodyTrait> {
    Cons {
        upleft: Box<BorrowerQuadtree<'l, T>>,
        upright: Box<BorrowerQuadtree<'l, T>>,
        downright: Box<BorrowerQuadtree<'l, T>>,
        downleft: Box<BorrowerQuadtree<'l, T>>,
    },
    Nil,
}



impl<'l, T: 'l + BodyTrait> BorrowerQuadtree<'l, T> {
    /// create a new BorrowerQuadtree 
    pub fn new(x: f64, y: f64, width: f64, height: f64, max_obj: usize, max_lvl: usize) -> BorrowerQuadtree<'l, T> {

        BorrowerQuadtree {
            level: max_lvl,
            max_object: max_obj,

            objects: vec![],

            x: x,
            y: y,

            width: width,
            height: height,

            nodes: BorrowerQuadtreeBatch::Nil,
        }
    }

    fn split(&mut self) {
        let max_lvl = self.level;
        let max_obj = self.max_object;

        let sub_width = self.width/2.;
        let sub_height = self.height/2.;
        let x = self.x;
        let y = self.y;

        let mut downleft = Box::new(BorrowerQuadtree::new(x, y, sub_width, sub_height, max_obj, max_lvl-1));
        let mut downright = Box::new(BorrowerQuadtree::new(x+sub_width, y, sub_width, sub_height, max_obj, max_lvl-1));
        let mut upright = Box::new(BorrowerQuadtree::new(x+sub_width, y+sub_height, sub_width, sub_height, max_obj, max_lvl-1));
        let mut upleft = Box::new(BorrowerQuadtree::new(x, y+sub_height, sub_width, sub_height, max_obj, max_lvl-1));

        let center_x = self.x + self.width/2.;
        let center_y = self.y + self.height/2.;

        let mut objects = Vec::<&'l T>::new();
        objects.append(&mut self.objects);

        while let Some(obj) = self.objects.pop() {

            match quadrant!(center_x,center_y,obj) {
                Quadrant::Upright => upright.insert(obj),
                Quadrant::Upleft => upleft.insert(obj),
                Quadrant::Downright => downright.insert(obj),
                Quadrant::Downleft => downleft.insert(obj),
                Quadrant::Nil => self.objects.push(obj),
            }

        }

        self.nodes = BorrowerQuadtreeBatch::Cons {
            downleft: downleft,
            downright: downright,
            upright: upright,
            upleft: upleft,
        };
    }

    pub fn insert(&mut self, obj: &'l T) {
        if self.level == 0 {
            self.objects.push(obj);
            return;
        }
        if let BorrowerQuadtreeBatch::Nil = self.nodes {
            if self.max_object == self.objects.len() {
                self.split();
            } else {
                self.objects.push(obj);
                return;
            }
        }

        if let BorrowerQuadtreeBatch::Cons { ref mut upleft, ref mut upright, ref mut downright, ref mut downleft } = self.nodes {
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

    pub fn insert_and_apply<F: Fn(&T)>(&mut self, obj: &'l T, callback: &F) {
        for obj in &self.objects {
            callback(obj);
        }

        if self.level == 0 {
            self.objects.push(obj);
            return;
        }
        if let BorrowerQuadtreeBatch::Nil = self.nodes {
            if self.max_object == self.objects.len() {
                self.split();
            } else {
                self.objects.push(obj);
                return;
            }
        }
        if let BorrowerQuadtreeBatch::Cons { ref mut upleft, ref mut upright, ref mut downright, ref mut downleft } = self.nodes {

            let center_x = self.x + self.width/2.;
            let center_y = self.y + self.height/2.;

            match quadrant!(center_x,center_y,obj) {
                Quadrant::Upright => upright.insert_and_apply(obj,callback),
                Quadrant::Upleft => upleft.insert_and_apply(obj,callback),
                Quadrant::Downright => downright.insert_and_apply(obj,callback),
                Quadrant::Downleft => downleft.insert_and_apply(obj,callback),
                Quadrant::Nil => { 
                    upright.apply(callback);
                    upleft.apply(callback);
                    downright.apply(callback);
                    downleft.apply(callback);
                },
            };
        } else {
            panic!("quadtree: insert_and_get_id split failed");
        }
    }

    pub fn apply<F: Fn(&T)>(&self, callback: &F) {
        for obj in &self.objects {
            callback(&*obj);
        }
        if let BorrowerQuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {
            upright.apply(callback);
            upleft.apply(callback);
            downright.apply(callback);
            downleft.apply(callback);
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

        if let BorrowerQuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {
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

    /// return a fixed quadtree that have the same structure as self but only store ids 
    /// and not point pointer on objects
    pub fn fix(&self) -> FixedQuadtree {
        let mut fixed = FixedQuadtree::new(self.x,self.y,self.width,self.height);
        fixed.ids = self.objects.iter().map(|&obj| obj.id()).collect::<Vec<usize>>();
    
        if let BorrowerQuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {
            fixed.nodes = FixedQuadtreeBatch::Cons {
                downleft:Box::new(downleft.fix()),
                downright:Box::new(downright.fix()),
                upright:Box::new(upright.fix()),
                upleft:Box::new(upleft.fix()),
    
            };
        } 
    
        fixed
    }
}
