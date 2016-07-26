use specs;
use entities;
use config;
use std::path::Path;
use specs::Join;
use physic;
use toml;
use bmp;
use baal;
use configuration;

#[derive(Clone)]
pub enum Level {
    Dungeon(usize,usize),
    Entry,
}
impl Level {
    fn next(&self) -> Self {
        if let Level::Dungeon(dungeon_id,room_id) = *self {
            let dungeon = config.levels.dungeons.get(dungeon_id).expect("INTERN ERROR: false dungeon)");

            if room_id >= dungeon.rooms.len() { panic!("INTERN ERROR: false room") }

            if room_id + 1 == dungeon.rooms.len() {
                Level::Entry
            } else {
                Level::Dungeon(dungeon_id,room_id+1)
            }
        } else {
            panic!("INTERN ERROR: cannot call next on entry dungeon");
        }
    }
}

pub struct Dungeon {
    name: String,
    music: usize,
    rooms: Vec<String>,
}
impl configuration::FromToml for Dungeon {
    fn from_toml(value: &toml::Value) -> Result<Self,String> {
        let table = try!(value.as_table().ok_or(String::from(" expect table")));
        let mut name = None;
        let mut music = None;
        let mut rooms = None;

        for (key,value) in table {
            match &**key {
                "name" => if name.is_none() {
                    name = Some(String::from(try!(value.as_str().ok_or(String::from(" expect name to be a string")))));
                } else {
                    return Err(String::from(" double definition of name"));
                },
                "rooms" => if rooms.is_none() {
                    let mut vec = vec!();
                    let mut i = 0;
                    for value in try!(value.as_slice().ok_or(String::from(" expect rooms to be an array"))) {
                        vec.push(String::from(try!(value.as_str().ok_or(format!(" expect rooms[{}] to be a string",i)))));
                        i += 1;
                    }
                    rooms = Some(vec);
                } else {
                    return Err(String::from(" double definition of rooms"));
                },
                "music" => if music.is_none() {
                    let v = try!(value.as_integer().ok_or(String::from(" expect music to be an integer")));
                    if v >= 0 {
                        music = Some(v as usize);
                    } else {
                        return Err(String::from(" expect music to be a positive integer"));
                    }
                } else {
                    return Err(String::from(" double definition of music"));
                },
                _ => return Err(String::from(" uneepected key")),
            }
        }

        Ok(Dungeon {
            name: try!(name.ok_or(String::from(" expect name key"))),
            music: try!(music.ok_or(String::from(" expect music key"))),
            rooms: try!(rooms.ok_or(String::from(" expect rooms key"))),
        })
    }
}

#[derive(Debug)]
pub enum LoadError {
    ComputeLevel(usize,usize),
    OpenBmp(bmp::BmpError),
    UnexpectedColor,
}

pub fn load<'l>(level: &Level, world: &mut specs::World) -> Result<specs::Entity,LoadError> {
    // flush world
    for entity in world.entities().iter() {
        world.delete_later(entity);
    }
    world.maintain();

    // read level file
    if let &Level::Dungeon(dungeon,room) = level {
        let dungeon = try!(config.levels.dungeons.get(dungeon).ok_or(LoadError::ComputeLevel(dungeon,room)));

        if let Some(music) = baal::music::status().id {
            if music != dungeon.music {
                baal::music::play(dungeon.music);
            }
        }

        let path = Path::new(&*config.levels.dir).join(Path::new(&*dungeon.name).join(Path::new(&*format!("{}{}",room,".bmp"))));
        let image = try!(bmp::open(&*path.to_string_lossy()).map_err(|e| LoadError::OpenBmp(e)));
        for (x,y) in image.coordinates() {
            let pixel = image.get_pixel(x,y);
            let col = [pixel.r,pixel.g,pixel.b];
            if col == config.levels.empty_col {
            } else if col == config.levels.char_col {
                entities::add_character(world,[x as isize,y as isize]);
            } else if col == config.levels.portal_col {
                entities::add_portal(world,[x as isize,y as isize],level.next());
            } else if col == config.levels.laser_col {
                entities::add_laser(world,[x as isize,y as isize]);
            } else if col == config.levels.monster_col {
                entities::add_monster(world,[x as isize,y as isize]);
            } else if col == config.levels.column_col {
                entities::add_column(world,[x as isize,y as isize]);
            } else if col == config.levels.wall_col {
                entities::add_wall(world,[x as isize,y as isize]);
            } else {
                return Err(LoadError::UnexpectedColor);
            }
        }
    } else {
        if let Some(music) = baal::music::status().id {
            if music != config.levels.entry_music {
                baal::music::play(config.levels.entry_music);
            }
        }

        entities::add_character(world,[0,0]);

        entities::add_wall(world,[-1,-1]);
        entities::add_wall(world,[-1,0]);
        entities::add_wall(world,[-1,1]);

        entities::add_wall(world,[0,-1]);
        entities::add_wall(world,[0,1]);

        entities::add_wall(world,[1,-1]);
        entities::add_laser(world,[1,0]);
        entities::add_wall(world,[1,1]);

        entities::add_wall(world,[2,1]);
        entities::add_wall(world,[3,-1]);
        entities::add_wall(world,[3,1]);
        entities::add_wall(world,[4,-1]);
        entities::add_wall(world,[4,1]);
        entities::add_wall(world,[5,1]);
        entities::add_wall(world,[5,0]);
        entities::add_wall(world,[5,-1]);
        entities::add_laser(world,[3,0]);

        if config.levels.dungeons.len() != 0 {
            entities::add_portal(world,[4,0],Level::Dungeon(0,0));
        }
        for i in (1..config.levels.dungeons.len()).map(|x| (x*2) as isize) {
            entities::add_wall(world,[1,-i]);
            entities::add_wall(world,[1,-1-i]);
            entities::add_wall(world,[3,-1-i]);
            entities::add_wall(world,[4,-1-i]);
            entities::add_wall(world,[5,-1-i]);
            entities::add_wall(world,[5,-i]);
            entities::add_laser(world,[3,-i]);
        }
        entities::add_wall(world,[2,-1-(config.levels.dungeons.len() as isize)*2]);
    }

    // add_physic_world
    let master_entity = world.create_now()
        .with::<physic::PhysicWorld>(physic::PhysicWorld::new())
        .build();

    // init_physic_world
    let mut physic_worlds = world.write::<physic::PhysicWorld>();
    physic_worlds.get_mut(master_entity).unwrap().fill(&world);

    Ok(master_entity)
}
