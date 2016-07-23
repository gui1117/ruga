use specs;
use entities;
use hlua;
use hlua::Lua;
use std::fs::File;
use config;
use std::path::Path;
use std::io;
use specs::Join;
use physic;
use rand;
use rand::distributions::{IndependentSample, Range};
use std::collections::HashMap;

#[derive(Debug)]
pub enum LoadError {
    OpenFile(io::Error),
    Lua(hlua::LuaError),
}

enum Cell {
    Character,
    Wall,
    Column,
    Monster,
    Laser,
    Portal(String),
}

pub fn load<'l>(level: &str, world: &specs::World) -> Result<specs::Entity,LoadError>{
    // flush world
    for entity in world.entities().iter() {
        world.delete_later(entity);
    }
    world.maintain();

    let mut cells = HashMap::new();

    // init lua level creation context
    let mut lua: Lua<'l> = Lua::new();
    lua.set("add_character", hlua::function2(|x: i32,y: i32| {
        cells.insert((x,y),Cell::Character);
    }));
    lua.set("add_wall", hlua::function2(|x: i32,y: i32| {
        cells.insert((x,y),Cell::Wall);
    }));
    lua.set("add_column", hlua::function2(|x: i32,y: i32| {
        cells.insert((x,y),Cell::Column);
    }));
    lua.set("add_monster", hlua::function2(|x: i32,y: i32| {
        cells.insert((x,y),Cell::Monster);
    }));
    lua.set("add_laser", hlua::function2(|x: i32,y: i32| {
        cells.insert((x,y),Cell::Laser);
    }));
    lua.set("add_portal", hlua::function3(|x: i32,y: i32,dest: String| {
        cells.insert((x,y),Cell::Portal(dest));
    }));
    lua.set("generate_kruskal", hlua::function3(|width: i32, height: i32, percent: f64| -> Vec<Vec<bool>> {
        assert!(width >= 0);
        assert!(height >= 0);
        generate_partial_reverse_randomized_kruskal(width as usize, height as usize, percent)
    }));

    // execute common script
    let path = Path::new(&*config.levels.dir).join(Path::new(&*format!("{}{}",config.levels.common,".lua")));
    let file = try!(File::open(&path).map_err(|e| LoadError::OpenFile(e)));
    try!(lua.execute_from_reader::<(),_>(file).map_err(|e| LoadError::Lua(e)));

    // execute level script
    let path = Path::new(&*config.levels.dir).join(Path::new(&*format!("{}{}",level,".lua")));
    let file = try!(File::open(&path).map_err(|e| LoadError::OpenFile(e)));
    try!(lua.execute_from_reader::<(),_>(file).map_err(|e| LoadError::Lua(e)));

    // fill world
    for ((x,y),cell) in cells.drain() {
        match cell {
            Cell::Character => entities::add_character(world,[x as isize,y as isize]),
            Cell::Portal(dest) => entities::add_portal(world,[x as isize,y as isize],dest),
            Cell::Laser => entities::add_laser(world,[x as isize,y as isize]),
            Cell::Monster => entities::add_monster(world,[x as isize,y as isize]),
            Cell::Column => entities::add_column(world,[x as isize,y as isize]),
            Cell::Wall=> entities::add_wall(world,[x as isize,y as isize]),
        }
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

/// https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_Kruskal.27s_algorithm
fn generate_partial_reverse_randomized_kruskal(width: usize, height: usize, percent: f64) -> Vec<Vec<bool>> {
    enum WallPos {
        Vertical(usize,usize),
        Horizontal(usize,usize),
    }

    assert_eq!(width.wrapping_rem(2), 1);
    assert_eq!(height.wrapping_rem(2), 1);

    let index = |x: usize, y: usize| y*width+x;

    let mut grid = Vec::with_capacity(width*height);
    for i in 0..width*height {
        grid.push((false,i));
    }

    for i in 0..width {
        grid[i] = (true, i);
        let j = height*(width-1)+i;
        grid[j] = (true, j);
    }

    for i in 0..height {
        grid[i*width] = (true, i*width);
        let j = (i+1)*width - 1;
        grid[j] = (true,j);
    }

    let horizontal_wall = (width-5)/2 * (height-3)/2;
    let vertical_wall = (width-3)/2 * (height-5)/2;
    let horizontal_wall_width = (width-5)/2;
    let vertical_wall_width = (width-3)/2;

    let mut walls = Vec::with_capacity(horizontal_wall+vertical_wall);
    for i in 0..vertical_wall {
        walls.push(WallPos::Vertical(i.wrapping_rem(vertical_wall_width)*2+2, (i/vertical_wall_width)*2+3));
    }
    for i in 0..horizontal_wall {
        walls.push(WallPos::Horizontal(i.wrapping_rem(horizontal_wall_width)*2+3, (i/horizontal_wall_width)*2+2));
    }

    let mut rng = rand::thread_rng();

    let stop = ((walls.len() as f64)*(1.-percent/100.)) as usize;

    while walls.len() > stop {
        let i = Range::new(0,walls.len()).ind_sample(&mut rng);
        assert!(i<walls.len());
        let (c1,c2,c3) = match walls.swap_remove(i) {
            WallPos::Vertical(x,y) => {
                (index(x,y-1), index(x,y), index(x,y+1))
            },
            WallPos::Horizontal(x,y) => {
                (index(x-1,y), index(x,y), index(x+1,y))
            },
        };

        let ((_,s1),(_,s2),(_,s3)) = (grid[c1],grid[c2],grid[c3]);

        if s1 != s3 {
            grid[c1] = (true,s1);
            grid[c2] = (true,s2);
            grid[c3] = (true,s3);
            for &mut(_,ref mut s) in &mut grid {
                if *s == s2 || *s == s3 {
                    *s = s1;
                }
            }
        }
    }

    let mut res = Vec::with_capacity(width);
    for i in 0..width {
        res.push(Vec::with_capacity(height));
        for j in 0..height {
            res[i].push(grid[index(i,j)].0);
        }
    }
    res
}

#[test]
fn test_levels() {
    use std::collections::HashSet;

    let mut visited = HashSet::new();
    let mut to_visit = vec!(config.levels.first_level.clone());

    loop {
        let level = match to_visit.pop() {
            Some(level) => level,
            None => break,
        };
        // init lua level creation context
        let mut lua = Lua::new();
        lua.set("add_character", hlua::function2(|_: i32,_: i32| {
        }));
        lua.set("add_wall", hlua::function2(|_: i32,_: i32| {
        }));
        lua.set("add_column", hlua::function2(|_: i32,_: i32| {
        }));
        lua.set("add_monster", hlua::function2(|_: i32,_: i32| {
        }));
        lua.set("add_laser", hlua::function2(|_: i32,_: i32| {
        }));
        lua.set("generate_kruskal", hlua::function3(|width: i32, height: i32, percent: f64| -> Vec<Vec<bool>> {
            generate_partial_reverse_randomized_kruskal(width as usize, height as usize, percent)
        }));
        lua.set("add_portal", hlua::function3(|_: i32,_: i32,dest: String| {
            if !visited.contains(&dest) {
                to_visit.push(dest);
            }
        }));

        // execute common script
        let path = Path::new(&*config.levels.dir).join(Path::new(&*format!("{}{}",config.levels.common,".lua")));
        let file = File::open(&path).map_err(|e| LoadError::OpenFile(e)).unwrap();
        lua.execute_from_reader::<(),_>(file).map_err(|e| LoadError::Lua(e)).unwrap();

        // execute level script
        let path = Path::new(&*config.levels.dir).join(Path::new(&*format!("{}{}",level,".lua")));
        let file = File::open(&path).map_err(|e| LoadError::OpenFile(e)).unwrap();
        lua.execute_from_reader::<(),_>(file).map_err(|e| LoadError::Lua(e)).unwrap();

        visited.insert(level);
    }
}
