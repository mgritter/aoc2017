const INPUT : &str = "R4, R4, L1, R3, L5, R2, R5, R1, L4, R3, L5, R2, L3, L4, L3, R1, R5, R1, L3, L1, R3, L1, R2, R2, L2, R5, L3, L4, R4, R4, R2, L4, L1, R5, L1, L4, R4, L1, R1, L2, R5, L2, L3, R2, R1, L194, R2, L4, R49, R1, R3, L5, L4, L1, R4, R2, R1, L5, R3, L5, L4, R4, R4, L2, L3, R78, L5, R4, R191, R4, R3, R1, L2, R1, R3, L1, R3, R4, R2, L2, R1, R4, L5, R2, L2, L4, L2, R1, R2, L3, R5, R2, L3, L3, R3, L1, L1, R5, L4, L4, L2, R5, R1, R4, L3, L5, L4, R5, L4, R5, R4, L3, L2, L5, R4, R3, L3, R1, L5, R5, R1, L3, R2, L5, R5, L3, R1, R4, L5, R4, R2, R3, L4, L5, R3, R4, L5, L5, R4, L4, L4, R1, R5, R3, L1, L4, L3, L4, R1, L5, L1, R2, R2, R4, R4, L5, R4, R1, L1, L1, L3, L5, L2, R4, L3, L5, L4, L1, R3";

#[derive(Debug)]
enum Heading {
    North,
    South,
    East,
    West
}

fn right( h : Heading ) -> Heading {
    match h {
        Heading::North => Heading::East,
        Heading::East => Heading::South,
        Heading::South => Heading::West,
        Heading::West => Heading::North
    }
}

fn left( h : Heading ) -> Heading {
    match h {
        Heading::North => Heading::West,
        Heading::East => Heading::North,
        Heading::South => Heading::East,
        Heading::West => Heading::South
    }
}

fn direction( h : &Heading ) -> (i32, i32) {
    // Dereference so we're comparing enum to enum, not enum to ref    
    match *h {
        Heading::North => ( 0, -1 ),
        Heading::East => ( 1, 0 ),
        Heading::South => ( 0, 1 ),
        Heading::West => ( -1, 0 )
    }
}

use std::collections::HashSet;

fn main() {
    let s = String::from( INPUT );
    let d : Vec<&str> = s.split( ", " ).collect();

    let mut h = Heading::North;
    let mut x = 0;
    let mut y = 0;

    let mut visited = HashSet::new();
    visited.insert( (0,0) );

    let mut first_repeat : Option<_> = None;
        
    for m in d.iter() {
        let (turn, blocks) = m.split_at( 1 );
        let blocks : i32 = blocks.parse().expect( "Couldn't parse." );
        
        println!( "{} {} {:?} {} {}", x, y, h, turn, blocks );

        if turn == "R" {
            h = right( h );
        } else if turn == "L" {
            h = left( h );
        } else {
            panic!( "Couldn't parse turn." );
        }
        
        let (dx, dy) = direction( &h );
        // OK, I tried to use ... which is deprecated
        // but ..= says "inclusive range syntax is experimental"
        // so back to the old standard with occasional off-by-one errors?
        for _ in 0..blocks {
            x += dx;
            y += dy;
            match first_repeat {
                None => {
                    if visited.contains( &(x,y) ) {
                        first_repeat = Some( (x, y) );
                    }
                },
                Some(_) => ()
            }
            visited.insert( (x,y) );
        }
    }

    println!( "Final {} {}", x, y );
    println!( "Distance: {}", x.wrapping_abs() + y.wrapping_abs() );
    if let Some((x,y)) = first_repeat {
        println!( "First repeat {} {}", x, y );
        println!( "Distance: {}", x.wrapping_abs() + y.wrapping_abs() );
    }
}
