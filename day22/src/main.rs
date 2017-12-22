const INPUT : &str = ".########.....#...##.####
....#..#.#.##.###..#.##..
##.#.#..#.###.####.##.#..
####...#...####...#.##.##
..#...###.#####.....##.##
..#.##.######.#...###...#
.#....###..##....##...##.
##.##..####.#.######...##
#...#..##.....#..#...#..#
........#.##..###.#.....#
#.#..######.#.###..#...#.
.#.##.##..##.####.....##.
.....##..#....#####.#.#..
...#.#.#..####.#..###..#.
##.#..##..##....#####.#..
.#.#..##...#.#####....##.
.####.#.###.####...#####.
...#...######..#.##...#.#
#..######...#.####.#..#.#
...##..##.#.##.#.#.#....#
###..###.#..#.....#.##.##
..#....##...#..#..##..#..
.#.###.##.....#.###.#.###
####.##...#.#....#..##...
#.....#.#..#.##.#..###..#";

use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let mut infected : HashMap<(i32,i32),bool> = HashMap::new();
    let mut height = 0;
    let mut width = 0;
    
    for (y, row) in INPUT.split( '\n' ).enumerate() {
        height = max( height, y );
        for (x, c) in row.chars().enumerate() {
            width = max( width, x );
            if c == '#' {
                infected.insert( (x as i32, y as i32), true );
            }
        }
    }

    let mut x : i32 = width as i32 / 2;
    let mut y : i32 = height as i32 / 2;
    println!( "Start position: {},{}", x, y );
    let mut dx = 0;
    let mut dy = -1;
    let mut infected_count = 0;
    
    for iter in 0..10000 {
        if infected.get( &(x,y) ).cloned().unwrap_or( false ) {
            // Turn right
            match (dx,dy) {
                (1,0) => { dx = 0; dy = 1; },
                (0,1) => { dx = -1; dy = 0; },
                (-1, 0) => { dx = 0; dy = -1; },
                (0,-1) => { dx = 1; dy = 0; }
                (_,_) => { panic!( "Unknown direction!" ); }
            }
            // Clean
            infected.insert( (x,y), false );
            
        } else {
            // Turn left
            match (dx,dy) {
                (1,0) => { dx = 0; dy = -1; },
                (0,1) => { dx = 1; dy = 0; },
                (-1, 0) => { dx = 0; dy = 1; },
                (0,-1) => { dx = -1; dy = 0; }
                (_,_) => { panic!( "Unknown direction!" ); }
            }

            // Infect
            infected.insert( (x,y), true );
            infected_count += 1;
        }
        x += dx;
        y += dy;
    }

    println!( "Cells infected: {}", infected_count );
}
