use specs;
use entities;
use hlua;

#[derive(Clone,Debug)]
pub enum LoadError {
}

fn load(level: String, world: &mut specs::World, entities: &mut entities::Setting) -> Result<(),LoadError>{
    unimplemented!();
}

