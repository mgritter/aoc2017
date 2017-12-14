const INPUT : &str = "oundnydw";

fn twist( x : & mut Vec<u32>, pos : usize, length : usize ) {
    let m = x.len();
    for i in 0..(length/2) {
        let src = (pos + i) % m;
        let dst = (pos + length - i - 1 ) % m;
        // For the life of me I can't figure out how to get swap to work,
        // too many mutable borrows I guess.
        let tmp = x[dst];
        x[dst] = x[src];
        x[src] = tmp;
    }
}

fn knot_hash( input_text : &str ) -> Vec<u8> {
    let mut input : Vec<usize> = input_text.as_bytes().iter().map( |c| *c as usize ).collect();
    let extra : Vec<usize> = vec!( 17, 31, 73, 47, 23 );
    input.extend( extra );

    let mut list : Vec<u32> = (0..256).collect();

    let mut pos : usize = 0;
    let mut skip_size : usize = 0;

    for _round in 0..64 {
        for length in input.iter() {
            twist( &mut list, pos, *length );
            // println!( "Twist: {} {:?}", length, list );
            pos = (pos + length + skip_size) % list.len();
            skip_size += 1;
            // println!( "Pos: {}, skip_size: {}", pos, skip_size );
        }
    }
    
    // println!( "Final list: {:?}", list );

    let mut hash : Vec<u8> = Vec::new();
    for i in 0..16 {
        // 8 bits at a time
        let hv = list[i*16..(i+1)*16].iter().fold( 0, |acc, &x| acc ^ x );
        hash.push( hv as u8 );
    }
    return hash;
}

fn count_set_bits( n : u8 ) -> u32 {
    let mut count = 0;
    let mut w = n;
    while w != 0 {
        w &= w-1;
        count += 1
    }
    count
}

fn neighbors( pos : (i32, i32) ) -> Vec<(i32,i32)> {
    let (x,y) = pos;
    vec!( (x+1,y), (x-1,y), (x,y+1), (x,y-1) )
}

use std::collections::HashSet;

fn bfs_region( present : & HashSet<(i32,i32)>,
               start : (i32,i32) )
               -> HashSet<(i32,i32)> {
    let mut visited : HashSet<(i32,i32)> = HashSet::new();
    let mut stack : Vec<(i32,i32)> = Vec::new();
    visited.insert( start );
    stack.push( start );
    while !stack.is_empty() {
        let a = stack.pop().unwrap();
        // println!( "visiting {}", a );

        for b in neighbors( a ).iter() {
            if present.contains( b ) && !visited.contains( b ) {
                visited.insert( *b );
                stack.push( *b );
            }
        }        
    }
    
    visited
}

                                        
fn difference_in_place( a : &mut HashSet<(i32,i32)>,
                        b : & HashSet<(i32,i32)> ) {
    for x in b {
        a.remove( x );
    }
}

fn main() {
    let mut count : u32 = 0;
    let mut bits : HashSet<(i32,i32)> = HashSet::new();
    
    for row in 0..128 {
        let hv = knot_hash( format!( "{}-{}", INPUT, row ).as_str() );
        let mut col = 0;
        for b in &hv {
            count += count_set_bits( *b );
            for offs in 0..8 {
                if *b & ( 1 << (7 - offs) ) != 0 {
                    bits.insert( (row, col + offs ) );
                }
            }
            col += 8;
        }
    }
    println!( "Bit count = {}", count );
    println!( "Bit count check = {}", bits.len() );

    let mut num_components = 0;

    while !bits.is_empty() {
        let root = bits.iter().next().unwrap().clone();
        let g_root = bfs_region( &bits, root );
        println!( "Root: {:?} size {}", root, g_root.len() );
        difference_in_place( &mut bits, &g_root );
        num_components += 1;
    }
    println!( "Number of components: {}", num_components );    
}
