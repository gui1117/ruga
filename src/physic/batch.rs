use super::quadtree::{ Quadtree, Localisable, Identifiable };

pub struct Batch<'l, B: 'l+Localisable+Identifiable>{
    vector: Vec<B>,
    quadtree: Quadtree<'l,B>,
}

impl<'l, B: 'l+Localisable+Identifiable> Batch<'l, B> {
    pub fn new() -> Batch<'l, B> {
        Batch {
            vector: vec![],
            quadtree: Quadtree::<'l, B>::new(0.,0.,100.,100.,2,2),
        }
    }

    pub fn insert(&'l mut self, body: B) {
        let id = body.id();
        self.vector[id] = body;
        self.quadtree.insert(&self.vector[id]);
    }

    pub fn get(id: usize) -> Option<&'l B> {
        None
    }

    pub fn remove(id: usize) -> Option<B> {
        None
    }

    pub fn raycast() {
    }

    pub fn query() {
    }
}

//#[cfg(test)]
//pub struct SimpleBody {
//    id: usize,
//    shape: super::geometry::Shape,
//}
//
//#[test]
//impl Identifiable for SimpleBody {
//    fn id(&self) -> usize {
//        self.id
//    }
//}
//
//#[test]
//impl Localisable for SimpleBody {
//    fn left(&self) -> bool {
//        self.shape.left()
//    }
//
//    fn right(&self) -> bool {
//        self.shape.right()
//    }
//
//    fn up(&self) -> bool {
//        self.shape.up()
//    }
//
//    fn down(&self) -> bool {
//        self.shape.down()
//    }
//}
//
//fn insert() {
//    let batch: Batch<Body> = Batch::new();
//}
