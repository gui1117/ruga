use specs;
use entities;
use hlua;
use hlua::Lua;
use hlua::any::AnyLuaValue;
use std::fs::File;
use std::path::Path;
use std::io;

#[derive(Debug)]
pub enum LoadError {
    OpenFile(io::Error),
    Lua(hlua::LuaError),
}


pub fn load<'l>(level: String, world: &mut specs::World, entities: &entities::Setting) -> Result<(),LoadError>{
    let file = try!(File::open(&Path::new("script.lua")).map_err(|e| LoadError::OpenFile(e)));

    let mut lua: Lua<'l> = Lua::new();

    lua.set("add_character", hlua::function0(|| {}));
        // entities.character.create(world)
        //     .with

    try!(lua.execute_from_reader::<(),_>(file).map_err(|e| LoadError::Lua(e)));

    Ok(())
}

