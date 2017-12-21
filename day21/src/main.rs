const INPUT : &str = "../.. => #.#/###/#.#
#./.. => ..#/.../###
##/.. => .../.##/###
.#/#. => #.#/##./.#.
##/#. => #../#.#/..#
##/## => #.#/#../###
.../.../... => .###/..##/.#../###.
#../.../... => ##.#/.###/#.../##.#
.#./.../... => ..../#.##/..../.#.#
##./.../... => ..#./#.../#.../.###
#.#/.../... => ..##/####/#.#./..##
###/.../... => .##./#.#./###./.#..
.#./#../... => #..#/..#./...#/#.#.
##./#../... => ##../.##./##.#/#..#
..#/#../... => #.##/.##./##.#/.###
#.#/#../... => ...#/#.##/..#./##..
.##/#../... => #.#./..#./##.#/.#.#
###/#../... => #..#/...#/..#./##.#
.../.#./... => .#.#/#.../.##./.#.#
#../.#./... => #.#./.##./..../.#.#
.#./.#./... => .#../#.##/..##/..##
##./.#./... => ..##/##.#/...#/..#.
#.#/.#./... => ...#/.##./####/.#..
###/.#./... => ###./####/...#/####
.#./##./... => ...#/.#.#/#.#./#.#.
##./##./... => ..../...#/#.#./...#
..#/##./... => .##./#.../##.#/.#..
#.#/##./... => .#../.#../...#/....
.##/##./... => ..#./.##./####/.#..
###/##./... => ##../.#.#/##../.#..
.../#.#/... => ..#./.#../.#../.###
#../#.#/... => ####/..../#..#/#...
.#./#.#/... => #.##/##../##.#/##.#
##./#.#/... => ###./..../#.##/###.
#.#/#.#/... => ###./.#../#.#./#.#.
###/#.#/... => ...#/#..#/#.#./..##
.../###/... => .#../...#/...#/....
#../###/... => ####/#.../##../.#.#
.#./###/... => ...#/####/.##./#..#
##./###/... => .###/##.#/..#./.#..
#.#/###/... => ####/###./.###/.###
###/###/... => .#.#/..##/..#./##..
..#/.../#.. => #.../.###/###./...#
#.#/.../#.. => ..../#.../##../..#.
.##/.../#.. => ####/####/...#/####
###/.../#.. => #.../.#../#.#./#.#.
.##/#../#.. => ##../..#./.#../##.#
###/#../#.. => ..../#..#/.###/.###
..#/.#./#.. => ...#/##../.##./##..
#.#/.#./#.. => #.##/.###/#.#./##.#
.##/.#./#.. => ..../..../.#.#/#..#
###/.#./#.. => ##../.#.#/.#.#/####
.##/##./#.. => #.##/##.#/####/....
###/##./#.. => ..../..##/#.#./.###
#../..#/#.. => #.#./...#/#.##/.###
.#./..#/#.. => ####/#.##/.#../.###
##./..#/#.. => .##./..#./.#.#/##.#
#.#/..#/#.. => .#.#/#.##/##../#...
.##/..#/#.. => ..../.###/####/.#..
###/..#/#.. => ##.#/##.#/..##/.#..
#../#.#/#.. => #.##/###./##../....
.#./#.#/#.. => ..../###./####/###.
##./#.#/#.. => ##.#/#.##/##../#.##
..#/#.#/#.. => .###/#.../.#../##..
#.#/#.#/#.. => ##../##.#/#.../.##.
.##/#.#/#.. => ...#/..#./.###/##.#
###/#.#/#.. => #.../#.##/..##/..##
#../.##/#.. => #.##/#.../.##./##..
.#./.##/#.. => #.#./#.../..##/.#..
##./.##/#.. => .###/.#.#/####/.#.#
#.#/.##/#.. => ####/.#../##.#/.###
.##/.##/#.. => .#../##.#/####/#.#.
###/.##/#.. => #.##/#.../...#/....
#../###/#.. => ###./.#.#/##../#..#
.#./###/#.. => #..#/..##/..../....
##./###/#.. => ..#./#.../...#/###.
..#/###/#.. => ##../..##/##../#.##
#.#/###/#.. => ..../..#./.###/##..
.##/###/#.. => #..#/####/.#.#/.##.
###/###/#.. => ###./#.##/##.#/.#..
.#./#.#/.#. => #.../####/#.#./.##.
##./#.#/.#. => ..##/..../.#.#/##..
#.#/#.#/.#. => ####/..##/####/#...
###/#.#/.#. => ##.#/#.#./.##./####
.#./###/.#. => .#.#/.#.#/##.#/###.
##./###/.#. => .#../###./#.##/#...
#.#/###/.#. => #.../.###/.#../.#..
###/###/.#. => #.#./.##./.###/####
#.#/..#/##. => .#../#..#/###./#.##
###/..#/##. => #.#./####/###./###.
.##/#.#/##. => .#.#/...#/..../#.##
###/#.#/##. => ...#/..../#.##/####
#.#/.##/##. => ##../.#../.#.#/##..
###/.##/##. => #.../#.#./#.#./#.#.
.##/###/##. => ..../#.##/#.##/..##
###/###/##. => ####/##.#/#..#/.##.
#.#/.../#.# => ##.#/.#.#/####/.##.
###/.../#.# => #..#/.#.#/#.../#..#
###/#../#.# => ..##/###./.###/..##
#.#/.#./#.# => #.##/#.#./##../...#
###/.#./#.# => ..#./.###/..##/#...
###/##./#.# => #.../...#/..##/.###
#.#/#.#/#.# => #..#/.#../...#/#..#
###/#.#/#.# => ###./#.../##../.##.
#.#/###/#.# => ...#/..#./...#/#..#
###/###/#.# => ###./####/.###/###.
###/#.#/### => .###/.#../..../##.#
###/###/### => #..#/.###/##../.##.";

use std::collections::HashMap;

fn rotate_right_2( tile: &str ) -> String {
    // Input:  ab/cd    ab  ca
    // Output: da/cb    cd  db
    let v : Vec<char> = tile.chars().collect();
    vec!( v[3], v[0], '/', v[4], v[1] ).iter().collect()
}

fn flip_vert_2( tile: &str ) -> String {
    // Input:  ab/cd
    // Output: ba/dc
    let v : Vec<char> = tile.chars().collect();
    vec!( v[1], v[0], '/', v[4], v[3] ).iter().collect()
}

fn rotate_right_3( tile: &str ) -> String {
    // Input:  abc/def/ghi
    //         012 456 89A
    // abc  gda
    // def  heb
    // ghi  ifc
    // 
    // Output: gda/heb/ifc
    let v : Vec<char> = tile.chars().collect();
    vec!( v[8], v[4], v[0], '/', v[9], v[5], v[1], '/', v[10], v[6], v[2] ).iter().collect()
}

fn flip_vert_3( tile: &str ) -> String {
    // Input:  abc/def/ghi
    //         012 456 89A
    // Output: cba/fed/ihg
    let v : Vec<char> = tile.chars().collect();
    vec!( v[2], v[1], v[0], '/', v[6], v[5], v[4], '/', v[10], v[9], v[8] ).iter().collect()
}

fn symmetries_2( tile: &str ) -> Vec<String> {
    let r = rotate_right_2( tile );
    let r2 = rotate_right_2( &r );
    let r3 = rotate_right_2( &r2 );
    vec!(
        flip_vert_2( tile ),
        flip_vert_2( &r ),
        flip_vert_2( &r2 ),
        flip_vert_2( &r3 ),
        String::from( tile ),
        r,
        r2,
        r3,
    )
}

fn symmetries_3( tile: &str ) -> Vec<String> {
    let r = rotate_right_3( tile );
    let r2 = rotate_right_3( &r );
    let r3 = rotate_right_3( &r2 );
    vec!(
        flip_vert_3( tile ),
        flip_vert_3( &r ),
        flip_vert_3( &r2 ),
        flip_vert_3( &r3 ),
        String::from( tile ),
        r,
        r2,
        r3,
    )
}

fn show_field( x : &Vec<Vec<char>> ) {
    for row in x.iter() {
        println!( "{}", (*row).iter().collect::<String>() );
    }
}

fn count_row( x : &Vec<char> ) -> usize {
    x.iter().map( |r| if *r == '#' { 1 } else { 0 } ).sum()
}
    
fn count_field( x : &Vec<Vec<char>> ) -> usize {
    x.iter().map( |r| count_row(r) ).sum()
}

fn assemble( tiles : &Vec<&str>,
             width : usize ) -> Vec<Vec<char>> {
    let mut out : Vec<Vec<char>> = Vec::new();
    let mut y = 0;
    let tile_size = if tiles[0].len() == 11 {
        3
    } else {
        4
    };
    
    for t in tiles.iter() {
         for (i, row) in t.split( '/' ).enumerate() {
            if out.len() <= y + i {
                out.push( Vec::new() );
            }
            let mut row_chars = row.chars().collect();
            out[y+i].append( &mut row_chars );
        }
        if out[y].len() == width {
            y += tile_size;
        }
    }

    assert_eq!( out.len(), width );
    out
}

fn main() {
    // OK, I'm pretty sure some sort of divide-and-conquer is possible here
    // but I'm not smart enough right now to make that work, so we're just
    // gonna brute force this puppy.  With string operations, no less.
    
    let mut two_rules : HashMap<String,&str> = HashMap::new();
    let mut three_rules : HashMap<String,&str> = HashMap::new();
    let mut field : Vec< Vec<char> > = vec!(
        vec!( '.', '#', '.' ),
        vec!( '.', '.', '#' ),
        vec!( '#', '#', '#' ),
    );

    for rule in INPUT.split( "\n" ) {
        let mut tok = rule.split( " => ");
        let left = tok.next().unwrap();
        let right = tok.next().unwrap();
        
        // Pre-rotate and flip every rule.
        if left.len() == 5 {
            for src in symmetries_2( left ) {
                two_rules.insert( src, right );
            }
        } else {
            for src in symmetries_3( left ) {
                three_rules.insert( src, right );
            }
        }
    }

    assert_eq!( two_rules.len(), 16 );
    assert_eq!( three_rules.len(), 512 );
    
    for iter in 0..18 {
        let mut output : Vec<&str> = Vec::new();
        let width;        
        if field.len() % 2 == 0 {
            println!( "Using 2x2 rules" );
            for i in 0..(field.len() / 2) {
                let y = 2 * i;
                for j in 0..(field.len() / 2) {
                    let x = 2 * j;
                    let key : String =
                        vec!( field[y][x], field[y][x+1],
                              '/',
                              field[y+1][x], field[y+1][x+1] )
                        .iter().collect();
                    output.push( two_rules.get( &key ).unwrap() );
                }
            }
            width = ( field.len() / 2 ) * 3;
        } else {
            println!( "Using 3x3 rules" );
            for i in 0..(field.len() / 3) {
                let y = 3 * i;
                for j in 0..(field.len() / 3) {
                    let x = 3 * j;
                    let key : String =
                        vec!( field[y][x], field[y][x+1], field[y][x+2],
                              '/',
                              field[y+1][x], field[y+1][x+1], field[y+1][x+2],
                              '/',
                              field[y+2][x], field[y+2][x+1], field[y+2][x+2], )
                        .iter().collect();
                    output.push( three_rules.get( &key ).unwrap() );
                }
            }
            width = ( field.len() / 3 ) * 4;
        }
        // println!( "Tiles: {:?}", output );
        field = assemble( &output, width );
        println!( "Iteration: {} ", iter + 1 );
        if width < 80 {
            show_field( &field );
        }
        println!( "Active: {}", count_field( &field ) );
    }
}

mod test {
    use symmetries_2;
    use symmetries_3;

    fn show_tile( t : &String ) {
        for x in t.split( '/' ) {
            println!( "{}", x )
        }
        println!( "" );
    }
    
    // #[test]
    fn test_3() {
        let x = ".#./..#/###";
        for s in symmetries_3( x ) {
            show_tile( &s );
        }
    }

    #[test]
    fn test_2() {
        let x = ".#/#.";
        for s in symmetries_2( x ) {
            show_tile( &s );
        }
    }
}
