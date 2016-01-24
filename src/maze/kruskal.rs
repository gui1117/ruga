//! https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_Kruskal.27s_algorithm

extern crate rand;

use self::rand::distributions::{IndependentSample, Range};
use world::World;

#[derive(Debug)]
enum Wall {
    Vertical(usize,usize),
    Horizontal(usize,usize),
}

fn generate_partial_reverse_randomized_kruskal(width: usize, height: usize, percent: f64) -> Vec<bool> {
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
        walls.push(Wall::Vertical(i.wrapping_rem(vertical_wall_width)*2+2, (i/vertical_wall_width)*2+3));
    }
    for i in 0..horizontal_wall {
        walls.push(Wall::Horizontal(i.wrapping_rem(horizontal_wall_width)*2+3, (i/horizontal_wall_width)*2+2));
    }

    let mut rng = rand::thread_rng();

    let stop = ((walls.len() as f64)*(1.-percent/100.)) as usize;

    while walls.len() > stop {
        let i = Range::new(0,walls.len()).ind_sample(&mut rng);
        assert!(i<walls.len());
        let (c1,c2,c3) = match walls.swap_remove(i) {
            Wall::Vertical(x,y) => {
                (index(x,y-1), index(x,y), index(x,y+1))
            },
            Wall::Horizontal(x,y) => {
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

    grid.iter().map(|&(b,_)| b).collect::<Vec<bool>>()
}

pub fn generate() -> World {
    let width = 17;
    let height = 17;
    let unit = 40.;
    let percent = 30.;

    let maze = generate_partial_reverse_randomized_kruskal(width,height,percent);

    let mut world = World::new(30.);

    for i in 0..maze.len() {
        if maze[i] {
            let x = (i.wrapping_rem(width) as f64)*unit + unit/2.;
            let y = ((i/width) as f64)*unit + unit/2.;

            world.insert_wall(x,y,unit,unit);
        }
    }

    world.insert_boid(unit*1.5,unit*1.5,0.);
    world.insert_boid(unit*1.5,unit*1.5,0.);
    world.insert_boid(unit*1.5,unit*1.5,0.);
    world.insert_boid(unit*1.5,unit*1.5,0.);
    world.insert_boid(unit*1.5,unit*1.5,0.);
    world.insert_boid(unit*1.5,unit*1.5,0.);
    world.insert_character(unit*1.5,unit*1.5,0.);

    world
}
