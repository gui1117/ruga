use graphics;
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
use std::fs;
use std::fmt;
use std::io::Read;
use configuration::FromToml;
use std::io;

type VecDungeonSetting = Vec<DungeonSetting>;
type VecString = Vec<String>;
pub struct CastleSetting {
    music: String,
    dungeons: Vec<DungeonSetting>,
}
impl_from_toml_for_struct!( CastleSetting {
    music: String,
    dungeons: VecDungeonSetting,
});
pub struct DungeonSetting {
    name: String,
    music: String,
    rooms: VecString,
}
impl_from_toml_for_struct!( DungeonSetting {
    name: String,
    music: String,
    rooms: VecString,
});

pub enum LoadCastlesError {
    IoError(io::Error),
    FileNameInvalidUTF8,
    FileContentInvalidUTF8,
    TomlError(Vec<toml::ParserError>),
    InvalidTomlValue(String),
    UnexpectedFile,
    OpenConfigError(io::Error),
    ReadDirError(io::Error),
}

impl fmt::Display for LoadCastlesError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::LoadCastlesError::*;
        match *self {
            IoError(ref e) => write!(fmt,"io error: {}",e),
            FileNameInvalidUTF8 => write!(fmt,"file name is invalid utf-8"),
            FileContentInvalidUTF8 => write!(fmt,"file content is invalid utf-8"),
            TomlError(ref error_vec) => {
                try!(write!(fmt,"toml errors"));
                for e in error_vec {
                    try!(write!(fmt,"\n\t[{},{}] {}",e.lo,e.hi,e.desc));
                }
                write!(fmt,"")
            },
            InvalidTomlValue(ref e) => write!(fmt,"invalid toml value: {}",e),
            UnexpectedFile => write!(fmt,"unexpected file in levels root directory"),
            OpenConfigError(ref e) => write!(fmt,"open config failed: {}",e),
            ReadDirError(ref e) => write!(fmt,"read dir failed: {}",e),
        }
    }
}

impl From<io::Error> for LoadCastlesError {
    fn from(e: io::Error) -> Self {
        LoadCastlesError::IoError(e)
    }
}

/// argument: vector of musics
/// return a vector of castle and a vector of music name
/// the order of music name in the vector correspond to the music id
/// in the castles definitions
pub fn load_castles(mut musics: Vec<String>) -> Result<(Vec<Castle>,Vec<String>),LoadCastlesError> {
    let mut castles = Vec::new();

    for dir_entry in try!(fs::read_dir(config.levels.dir.clone()).map_err(|e| LoadCastlesError::ReadDirError(e))) {
        let dir_entry = try!(dir_entry);

        let castle_name = try!(dir_entry.file_name().into_string().map_err(|_| LoadCastlesError::FileNameInvalidUTF8));

        if !try!(dir_entry.file_type()).is_dir() {
            return Err(LoadCastlesError::UnexpectedFile);
        }
        let mut file = try!(fs::File::open(dir_entry.path().join(String::from("config.toml"))).
                        map_err(|e| LoadCastlesError::OpenConfigError(e)));

        let mut file_string = String::new();

        try!(file.read_to_string(&mut file_string)
             .map_err(|_| LoadCastlesError::FileContentInvalidUTF8));

        let mut file_parser = toml::Parser::new(&*file_string);
        let toml_table = try!(file_parser.parse() .ok_or(LoadCastlesError::TomlError(file_parser.errors)));

        let castle_setting = try!(CastleSetting::from_toml(&toml::Value::Table(toml_table))
            .map_err(|e| LoadCastlesError::InvalidTomlValue(e)));

        let castle_music = format!("{}/{}/musics/{}.ogg",config.levels.dir,castle_name,castle_setting.music);

        musics.push(castle_music);

        let mut castle = Castle {
            music: musics.len()-1,
            name: castle_name,
            dungeons: vec!(),
        };

        for dungeon in castle_setting.dungeons {
            let dungeon_music = format!("{}/{}/musics/{}.ogg",config.levels.dir,castle.name,dungeon.music);
            let index = if let Some(index) = musics.iter().position(|m| m.eq(&dungeon_music)) {
                index
            } else {
                musics.push(dungeon_music);
                musics.len()-1
            };
            castle.dungeons.push(Dungeon {
                music: index,
                name: dungeon.name,
                rooms: dungeon.rooms,
            });
        }

        castles.push(castle);
    }

    Ok((castles,musics))
}


#[derive(Debug,Clone)]
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

#[derive(Debug)]
pub struct Castle {
    pub name: String,
    pub music: usize,
    pub dungeons: Vec<Dungeon>,
}
#[derive(Debug,Clone)]
pub struct Dungeon {
    pub name: String,
    pub music: usize,
    pub rooms: Vec<String>,
}
impl_from_toml_for_struct!( Dungeon {
    name: String,
    music: usize,
    rooms: VecString,
});

#[derive(Debug)]
pub enum LoadLevelError {
    GetCastleError,
    GetDungeonError,
    GetRoomError,
    OpenBmp(bmp::BmpError),
    AmbiguousLevelDefinition,
    NoLevelDefinition,
    InvalidUTF8,
    UnexpectedColor,
    IoError(io::Error),
}
impl fmt::Display for LoadLevelError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::LoadLevelError::*;
        match *self {
            GetCastleError => write!(fmt,"castle id out of bounds"),
            GetDungeonError => write!(fmt,"dungeon id out of bounds"),
            GetRoomError => write!(fmt,"room id out of bounds"),
            OpenBmp(ref bmp_error) => write!(fmt,"open bmp error: {}",bmp_error),
            UnexpectedColor => write!(fmt,"unexpected color in bmp file"),
            IoError(ref e) => write!(fmt,"io error: {}",e),
            AmbiguousLevelDefinition => write!(fmt,"ambiguous level definition: both .txt and .bmp file exists"),
            InvalidUTF8 => write!(fmt,"text level invalid UTF-8"),
            NoLevelDefinition => write!(fmt,"level doesn't exist"),
        }
    }
}

impl From<io::Error> for LoadLevelError {
    fn from(e: io::Error) -> Self {
        LoadLevelError::IoError(e)
    }
}

pub fn load_level<'l>(level: &Level, castles: &Vec<Castle>, world: &mut specs::World) -> Result<specs::Entity,LoadLevelError> {
    // flush world
    for entity in world.entities().iter() {
        world.delete_later(entity);
    }
    world.maintain();

    baal::effect::short::stop_all();
    baal::effect::persistent::clear_positions_for_all();
    baal::effect::persistent::update_volume_for_all();
    baal::effect::short::play(config.entities.portal_snd, baal::effect::listener());

    // read level file
    match level {
        &Level::Room { castle: castle_id, dungeon: dungeon_id, room: room_id } => {
            let castle = try!(castles.get(castle_id).ok_or(LoadLevelError::GetCastleError));
            let dungeon = try!(castle.dungeons.get(dungeon_id).ok_or(LoadLevelError::GetDungeonError));

            if let Some(music) = baal::music::index() {
                if music != dungeon.music {
                    baal::music::play(dungeon.music);
                }
            }

            let room = try!(dungeon.rooms.get(room_id).ok_or(LoadLevelError::GetRoomError));

            let txt_path = Path::new(&*config.levels.dir).join(Path::new(&*format!("{}/maps/{}{}",castle.name,room,".txt")));
            let bmp_path = Path::new(&*config.levels.dir).join(Path::new(&*format!("{}/maps/{}{}",castle.name,room,".bmp")));

            match (txt_path.exists(),bmp_path.exists()) {
                (true,true) => return Err(LoadLevelError::AmbiguousLevelDefinition),
                (false,false) => return Err(LoadLevelError::NoLevelDefinition),
                (true,false) => {
                    let mut text = String::new();
                    try!(try!(fs::File::open(txt_path)).read_to_string(&mut text).map_err(|_| LoadLevelError::InvalidUTF8));

                    create_text_level(level.next(castles),text,world);
                },
                (false,true) => {
                    let image = try!(bmp::open(&*bmp_path.to_string_lossy()).map_err(|e| LoadLevelError::OpenBmp(e)));
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
                            return Err(LoadLevelError::UnexpectedColor);
                        }
                    }
                },
            }
        },
        &Level::Corridor { castle: castle_id } => {
            let castle = try!(castles.get(castle_id).ok_or(LoadLevelError::GetCastleError));

            if let Some(music) = baal::music::index() {
                if music != castle.music {
                    baal::music::play(castle.music);
                }
            }

            let levels = castle.dungeons.iter().enumerate().map(|(i,dungeon)| {
                (
                    dungeon.name.clone(),
                    Level::Room {
                        castle: castle_id,
                        dungeon: i,
                        room: 0,
                    }
                )
            }).collect();

            create_corridor(Some(Level::Entry),levels,world);
        },
        &Level::Entry => {
            if let Some(music) = baal::music::index() {
                if music != 0 {
                    baal::music::play(0);
                }
            }

            let levels = castles.iter().enumerate().map(|(i,castle)| {
                (
                    castle.name.clone(),
                    Level::Corridor {
                        castle: i,
                    }
            )
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

fn create_text_level(next: Level, text: String, world: &mut specs::World) {
    let bottom = config.text.bottom as isize;
    let left = config.text.left as isize;
    let right = config.text.right as isize;

    entities::add_fixed_camera_text(world,text);

    entities::add_character(world,[left+1,bottom+1]);
    entities::add_portal(world,[right-7,bottom+1],next);

    entities::add_wall(world,[left,bottom+1]);
    entities::add_wall(world,[right-6,bottom+1]);

    for x in left..right-5 {
        entities::add_wall(world,[x,bottom+2]);
        entities::add_wall(world,[x,bottom]);
    }
}

fn create_corridor(back: Option<Level>, mut levels: Vec<(String,Level)>, world: &mut specs::World) {
    let corridor_length = config.levels.corridor_length as isize;
    let hall_length = config.levels.hall_length as isize;

    if let Some(back) = back {
        entities::add_character(world,[-corridor_length-1,0]);
        entities::add_portal(world,[-corridor_length-1,-2],back);
        entities::add_text(world,"Exit".into(),vec!(graphics::Line {
            x: -hall_length as i32 -corridor_length as i32 -3,
            y: -2,
            length: 2,
        }));

        for x in 1..corridor_length+2 {
            entities::add_wall(world,[-x,-1]);
            entities::add_wall(world,[-x,1]);
            entities::add_wall(world,[-x,-3]);
        }

        for y in -3..2 {
            entities::add_wall(world,[-2-corridor_length,y]);
        }

        for x in 0..hall_length {
            entities::add_wall(world,[x,1]);
            entities::add_wall(world,[x,if levels.len() >= 2 { 1-(levels.len() as isize)*2 } else { -3 }]);
        }
        if levels.len() == 0 {
            for y in -3..2 {
                entities::add_wall(world,[hall_length,y]);
            }
        } else if levels.len() == 1 {
            for y in -3..0 {
                entities::add_wall(world,[hall_length,y]);
            }
        }
        if levels.len() != 0 {
            for x in hall_length..hall_length+corridor_length+2 {
                entities::add_wall(world,[x,1]);
            }
        }
    } else {
        for x in hall_length..hall_length+corridor_length+2 {
            entities::add_wall(world,[x,3]);
        }
        entities::add_wall(world,[hall_length+corridor_length+1,2]);
        entities::add_character(world,[hall_length+corridor_length,2]);

        entities::add_wall(world,[-1,3]);
        entities::add_wall(world,[-1,2]);
        entities::add_wall(world,[-1,1]);

        if levels.len() == 1 {
            entities::add_wall(world,[-1,0]);
            entities::add_wall(world,[-1,-1]);
        } else if levels.len() > 1 {
            entities::add_wall(world,[-1,0]);
            entities::add_wall(world,[-1,-1]);
            entities::add_wall(world,[-1,-2]);
            entities::add_wall(world,[-1,-3]);
        }

        for x in 0..hall_length {
            entities::add_wall(world,[x,3]);
            entities::add_wall(world,[x,1-(levels.len() as isize)*2]);
        }

        for x in hall_length..hall_length+corridor_length+2 {
            entities::add_wall(world,[x,1]);
        }
    }

    for (i,(name,level)) in levels.drain(..).enumerate() {
        let y = -((i*2) as isize);

        entities::add_text(world,name,vec!(graphics::Line {
            x: hall_length as i32 + corridor_length as i32 + 3,
            y: y as i32,
            length: 15,
        }));

        if i != 0 && i != 1 {
            entities::add_wall(world,[-1,y]);
            entities::add_wall(world,[-1,y-1]);
        }
        for x in hall_length..hall_length+corridor_length+2 {
            entities::add_wall(world,[x,y-1]);
        }
        entities::add_wall(world,[hall_length+corridor_length+1,y]);

        entities::add_portal(world,[hall_length+corridor_length,y],level);
    }
}
