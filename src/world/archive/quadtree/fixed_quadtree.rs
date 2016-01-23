use viewport::Viewport;
use opengl_graphics::GlGraphics;

use world::{ 
    Camera,
};
use super::{
    Localisable,
    Identifiable,
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

/// like a quadtree but only store id instead of reference to objects
pub struct FixedQuadtree {
    pub ids: Vec<usize>,
    pub x: f64, //downleft
    pub y: f64, //downleft
    pub width: f64,
    pub height: f64,
    pub nodes: FixedQuadtreeBatch,
}

// like a nodebatch but for FixedQuadtree
pub enum FixedQuadtreeBatch {
    Cons {
        upleft: Box<FixedQuadtree>,
        upright: Box<FixedQuadtree>,
        downright: Box<FixedQuadtree>,
        downleft: Box<FixedQuadtree>,
    },
    Nil,
}

impl FixedQuadtree {
    /// create a new Quadtree 
    ///
    /// * x,y: downleft
    /// * width,height: size 
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> FixedQuadtree {
        FixedQuadtree {
            ids: vec![],
            x: x,
            y: y,
            width: width,
            height: height,
            nodes: FixedQuadtreeBatch::Nil,
        }
    }

    // return a Vec of Id of all the objects in the current Quadtree but
    // not in its sons 
    fn objects_ids(&self) -> Vec<usize> {
        let mut result = vec![];
        for id in &self.ids {
            result.push(*id);
        }
        result
    }

    // return a Vec of Id of all the objects in the current Quadtree and 
    // in its sons
    fn all_objects_ids(&self) -> Vec<usize> {
        let mut result = vec![];
        for id in &self.ids {
            result.push(*id);
        }

        if let FixedQuadtreeBatch::Cons{ref upleft,ref upright,ref downleft,ref downright} = self.nodes {
            let r = upleft.all_objects_ids();
            for id in r {
                result.push(id);
            }

            let r = upright.all_objects_ids();
            for id in r {
                result.push(id);
            }

            let r = downright.all_objects_ids();
            for id in r {
                result.push(id);
            }

            let r = downleft.all_objects_ids();
            for id in r {
                result.push(id);
            }
        }
        result
    }

    /// Return a Vec of Id of all
    /// the objects that can collide with it
    pub fn query<T:Localisable>(&self, obj: &T) -> Vec<usize> {
        if let FixedQuadtreeBatch::Nil = self.nodes {
            return self.objects_ids();
        }

        let mut ids = self.objects_ids();
        let mut other_ids: Vec<usize> = vec![];

        if let FixedQuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {

            let x = self.x + self.width/2.;
            let y = self.y + self.height/2.;

            match quadrant!(x,y,obj) {
                Quadrant::Upright => { other_ids = upright.query(obj); },
                Quadrant::Upleft => { other_ids = upleft.query(obj); },
                Quadrant::Downright => { other_ids = downright.query(obj); },
                Quadrant::Downleft => { other_ids = downleft.query(obj); },
                Quadrant::Nil => { 
                    let r = upright.all_objects_ids();
                    for i in r {
                        other_ids.push(i);
                    }

                    let r = upleft.all_objects_ids();
                    for i in r {
                        other_ids.push(i);
                    }

                    let r = downright.all_objects_ids();
                    for i in r {
                        other_ids.push(i);
                    }

                    let r = downleft.all_objects_ids();
                    for i in r {
                        other_ids.push(i);
                    }
                },
            }

        }

        for id in other_ids {
            ids.push(id);
        }
        ids
    }

    /// insert an object in the fixed quadtree,
    pub fn insert<T:Localisable+Identifiable>(&mut self, obj: &T) {
        if let FixedQuadtreeBatch::Nil = self.nodes {
            self.ids.push(obj.id());
        }
        if let FixedQuadtreeBatch::Cons { ref mut upleft, ref mut upright, ref mut downright, ref mut downleft } = self.nodes {

            let x = self.x + self.width/2.;
            let y = self.y + self.height/2.;

            match quadrant!(x,y,obj) {
                Quadrant::Upright => { upright.insert(obj); },
                Quadrant::Upleft => { upleft.insert(obj); },
                Quadrant::Downright => { downright.insert(obj); },
                Quadrant::Downleft => { downleft.insert(obj); },
                Quadrant::Nil => { self.ids.push(obj.id()); },
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

        if let FixedQuadtreeBatch::Cons { ref upleft, ref upright, ref downright, ref downleft } = self.nodes {
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

}
