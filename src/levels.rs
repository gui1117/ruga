use specs;
use entities::Entities;
use hlua;
use hlua::Lua;
// use hlua::any::AnyLuaValue;
use std::fs::File;
use std::path::Path;
use std::io;
use specs::Join;

#[derive(Debug)]
pub enum LoadError {
    OpenFile(io::Error),
    Lua(hlua::LuaError),
}


pub fn load<'l>(level: String, world: &specs::World, entities: &Entities) -> Result<(),LoadError>{
    // flush world
    for entity in world.entities().iter() {
        world.delete_later(entity);
    }
    world.maintain();

    let dir = "levels";
    let path = Path::new(dir).join(Path::new(&*format!("{}{}",level,".lua")));

    let file = try!(File::open(&path).map_err(|e| LoadError::OpenFile(e)));

    let mut lua: Lua<'l> = Lua::new();

    lua.set("add_character", hlua::function2(|x: f32,y: f32| {
        entities.character.build(world,[x,y]);
    }));
    lua.set("add_monster", hlua::function2(|x: f32,y: f32| {
        entities.monster.build(world,[x,y]);
    }));

    try!(lua.execute_from_reader::<(),_>(file).map_err(|e| LoadError::Lua(e)));

    Ok(())
}

