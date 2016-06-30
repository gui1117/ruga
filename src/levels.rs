use specs;
use entities;
use hlua;
use hlua::Lua;
// use hlua::any::AnyLuaValue;
use std::fs::File;
use config;
use std::path::Path;
use std::io;
use specs::Join;
use physic;

#[derive(Debug)]
pub enum LoadError {
    OpenFile(io::Error),
    Lua(hlua::LuaError),
}

pub fn load<'l>(level: String, world: &specs::World) -> Result<specs::Entity,LoadError>{
    // flush world
    for entity in world.entities().iter() {
        world.delete_later(entity);
    }
    world.maintain();

    // init lua level creation context
    let mut lua: Lua<'l> = Lua::new();
    lua.set("add_character", hlua::function2(|x: i32,y: i32| {
        entities::add_character(world,[x as isize,y as isize]);
    }));
    lua.set("add_wall", hlua::function2(|x: i32,y: i32| {
        entities::add_wall(world,[x as isize,y as isize]);
    }));
    lua.set("add_column", hlua::function2(|x: i32,y: i32| {
        entities::add_column(world,[x as isize,y as isize]);
    }));
    lua.set("add_monster", hlua::function2(|x: i32,y: i32| {
        entities::add_monster(world,[x as isize,y as isize]);
    }));
    lua.set("add_laser", hlua::function2(|x: i32,y: i32| {
        entities::add_laser(world,[x as isize,y as isize]);
    }));

    // execute level script
    let path = Path::new(&*config.levels.dir).join(Path::new(&*format!("{}{}",level,".lua")));
    let file = try!(File::open(&path).map_err(|e| LoadError::OpenFile(e)));
    try!(lua.execute_from_reader::<(),_>(file).map_err(|e| LoadError::Lua(e)));

    // add_physic_world
    let master_entity = world.create_now()
        .with::<physic::PhysicWorld>(physic::PhysicWorld::new())
        .build();

    // init_physic_world
    let mut physic_worlds = world.write::<physic::PhysicWorld>();
    physic_worlds.get_mut(master_entity).unwrap().fill(&world);

    Ok(master_entity)
}

