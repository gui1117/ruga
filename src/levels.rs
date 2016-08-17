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

// type VecDungeonSetting = Vec<DungeonSetting>;
type VecString = Vec<String>;
// pub struct CastleSetting {
//     music: String,
//     dungeons: Vec<DungeonSetting>,
// }
// impl_from_toml_for_struct!( CastleSetting {
//     music: String,
//     dungeons: VecDungeonSetting,
// });
// pub struct DungeonSetting {
//     name: String,
//     music: String,
//     rooms: VecString,
// }
// impl_from_toml_for_struct!( DungeonSetting {
//     name: String,
//     music: String,
//     rooms: VecString,
// });

#[derive(Clone)]
pub enum Level {
    Room {
        castle: usize,
        dungeon: usize,
        room: usize,
    },
    Corridor {
        castle: usize,
    },
    Entry,
}
impl Level {
    fn next(&self, castles: &Vec<Castle>) -> Self {
        if let Level::Room { castle: castle_id, dungeon: dungeon_id, room: room_id } = *self {
            let dungeon = castles.get(castle_id).expect("INTERN ERROR: false castle").dungeons.get(dungeon_id).expect("INTERN ERROR: false dungeon)");

            if room_id >= dungeon.rooms.len() { panic!("INTERN ERROR: false room") }

            if room_id + 1 == dungeon.rooms.len() {
                Level::Corridor { castle: castle_id }
            } else {
                Level::Room {
                    castle: castle_id,
                    dungeon: dungeon_id,
                    room: room_id+1,
                }
            }
        } else {
            panic!("INTERN ERROR: cannot call next on entry dungeon");
        }
    }
}

pub struct Castle {
    pub name: String,
    pub music: usize,
    pub dungeons: Vec<Dungeon>,
}
#[derive(Clone)]
pub struct Dungeon {
    name: String,
    music: usize,
    rooms: Vec<String>,
}
impl_from_toml_for_struct!( Dungeon {
    name: String,
    music: usize,
    rooms: VecString,
});

#[derive(Debug)]
pub enum LoadError {
    GetCastleError,
    GetDungeonError,
    GetRoomError,
    OpenBmp(bmp::BmpError),
    UnexpectedColor,
}

pub fn load<'l>(level: &Level, castles: &Vec<Castle>, world: &mut specs::World) -> Result<specs::Entity,LoadError> {
    // flush world
    for entity in world.entities().iter() {
        world.delete_later(entity);
    }
    world.maintain();

    // read level file
    match level {
        &Level::Room { castle: castle_id, dungeon: dungeon_id, room: room_id } => {
            let castle = try!(castles.get(castle_id).ok_or(LoadError::GetCastleError));
            let dungeon = try!(castle.dungeons.get(dungeon_id).ok_or(LoadError::GetDungeonError));

            if let Some(music) = baal::music::index() {
                if music != dungeon.music {
                    baal::music::play(dungeon.music);
                }
            }

            let room = try!(dungeon.rooms.get(room_id).ok_or(LoadError::GetRoomError));
            let path = Path::new(&*config.levels.dir).join(Path::new(&*format!("{}/maps/{}{}",castle.name,room,".bmp")));
            let image = try!(bmp::open(&*path.to_string_lossy()).map_err(|e| LoadError::OpenBmp(e)));
            for (x,y) in image.coordinates() {
                let pixel = image.get_pixel(x,y);
                let col = [pixel.r,pixel.g,pixel.b];
                if col == config.levels.empty_col {
                } else if col == config.levels.char_col {
                    entities::add_character(world,[x as isize,y as isize]);
                } else if col == config.levels.portal_col {
                    entities::add_portal(world,[x as isize,y as isize],level.next(castles));
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
        },
        &Level::Corridor { castle: castle_id } => {
            let castle = try!(castles.get(castle_id).ok_or(LoadError::GetCastleError));

            if let Some(music) = baal::music::index() {
                if music != castle.music {
                    baal::music::play(castle.music);
                }
            }

            let levels = (0..castle.dungeons.len()).map(|i| Level::Room {
                castle: castle_id,
                dungeon: i,
                room: 0,
            }).collect();

            create_corridor(Some(Level::Entry),levels,world);
        },
        &Level::Entry => {
            if let Some(music) = baal::music::index() {
                if music != config.levels.entry_music {
                    baal::music::play(config.levels.entry_music);
                }
            }

            let levels = (0..castles.len()).map(|i| Level::Corridor {
                castle: i,
            }).collect();

            create_corridor(None,levels,world);
        },
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

fn create_corridor(back: Option<Level>, mut levels: Vec<Level>, world: &mut specs::World) {
    entities::add_character(world,[2,0]);

    entities::add_wall(world,[1,-1]);
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

    if let Some(back) = back {
        entities::add_portal(world,[0,0],back);

        entities::add_wall(world,[-1,-1]);
        entities::add_wall(world,[-1,0]);
        entities::add_wall(world,[-1,1]);

        entities::add_wall(world,[0,-1]);
        entities::add_wall(world,[0,1]);

        entities::add_laser(world,[1,0]);
    } else {
        entities::add_wall(world,[1,0]);
    }

    entities::add_wall(world,[2,1-(levels.len() as isize)*2]);

    for (i,level) in levels.drain(..).enumerate() {
        let y = -((i*2) as isize);
        if i != 0 {
            entities::add_wall(world,[1,y]);
            entities::add_wall(world,[1,y-1]);
        }
        entities::add_wall(world,[3,y-1]);
        entities::add_wall(world,[4,y-1]);
        entities::add_wall(world,[5,y-1]);
        entities::add_wall(world,[5,y]);
        entities::add_laser(world,[3,y]);

        entities::add_portal(world,[4,y],level);
    }
}
