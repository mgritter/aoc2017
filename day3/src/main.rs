const INPUT : i32 = 368078;
//const INPUT : i32 = 24;

// Ring 0 has 1 cell
// Ring 1 has 4*1 + 4 = 8 cells (2 through 9 = 3^2 )
// Ring 2 has 4*3 + 4 = 16 cells (10 through 25 = 5^2 )
// Ring 3 has 4*5 + 4 = 24 cells (26 through 49 = 7^2 )
// Ring 4 has 4*7 + 4 = 32 cells (50 through 81 = 9^2 )
// Ring N ends with (2N+1)^2

// 368078 = (606.7)^2
// 605^2 < 368078 < 607^2

use std::cmp::min;

fn square( x : i32 ) -> i32 {
    return x * x;
}

fn main() {
    let sqrt = (INPUT as f64).sqrt() as i32;
    if sqrt * sqrt == INPUT && sqrt % 2 == 1 {
        // At coordinates (-ring,ring)
        let ring = (sqrt - 1) / 2;
        println!( "Square, ring {}, distance {}", ring, 2 * ring );
        return ();
    }

    // 3 and 4 are in ring 2, 5 and 6 are in ring 3
    // once we've eliminated the exact match above
    let ring = ( sqrt + 1 ) / 2;
    let max_cell = square( 2 * ring + 1 );
    let half_width = ring;
    println!( "In ring {}", ring );
    
    // All these points are "ring" distance away, other points on the
    // ring are further
    
    // (0,-ring)
    let neg_y_axis = max_cell - half_width;
    // (-ring, 0)
    let neg_x_axis = max_cell - 3 * half_width;
    // (0, ring)
    let pos_y_axis = max_cell - 5 * half_width;
    // (ring,0)
    let pos_x_axis = max_cell - 7 * half_width;

    println!( "axes: {} {} {} {}", neg_y_axis, neg_x_axis, pos_y_axis, pos_x_axis );

    // Now, which is our number closest to?
    let axes = [ neg_y_axis, neg_x_axis, pos_y_axis, pos_x_axis ];    
    let min_dist = axes.iter()
        .map( |i| ( INPUT - i ).abs() )
        .fold( 2 * half_width, |acc, x| min( acc, x ) );
    println!( "Distance from axis {}", min_dist );
    println!( "Distance from origin {}", min_dist + ring );

    println!( "Second half solution {}", second_half() );
}

use std::collections::HashMap;

fn val_or_zero( h : &mut HashMap<(i32,i32),i32>,
                 px : i32,
                 py : i32 ) -> i32 {
    match h.get( &(px,py) ) {
        Some(v) => *v,
        None => 0
    }
}

fn sum_neighbors( h : &mut HashMap<(i32,i32),i32>,
                 px : i32,
                 py : i32 ) -> i32 {
    let sum = val_or_zero( h, px -1, py -1 ) +
        val_or_zero( h, px - 1, py ) +
        val_or_zero( h, px - 1, py + 1) +
        val_or_zero( h, px, py - 1 ) +
        val_or_zero( h, px, py + 1 ) +
        val_or_zero( h, px + 1, py - 1 ) +
        val_or_zero( h, px + 1, py) +
        val_or_zero( h, px + 1, py + 1 );
    println!( "Visiting {},{} = {}", px, py, sum );
    return sum;
    
}

fn second_half() -> i32 {
    // So much for being clever.
    // This is pretty ugly.
    //
    // The sequence is A141481 in the OEIS, so we could have just copied the
    // table at https://oeis.org/A141481/b141481.txt and done a lookup.
    // No compact representation is given there.
    
    let mut visited = HashMap::new();
    visited.insert( (0,0), 1 );
    let mut px = 1;
    let mut py = 0;
    for ring in 1..300 {
        println!( "Up ring {}", ring );
        for y in py..(px+1) {
            let val = sum_neighbors( &mut visited, px, y );
            if val > INPUT {
                return val;
            }
            visited.insert( (px, y), val );
        }
        py = px;
        println!( "Left ring {}", ring );
        for x in ((-px)..px).rev() {
            let val = sum_neighbors( &mut visited, x, py );
            if val > INPUT {
                return val;
            }
            visited.insert( (x, py), val );
        }
        px = -px;
        println!( "Down ring {}", ring );
        for y in ((-py)..py).rev() {
            let val = sum_neighbors( &mut visited, px, y );
            if val > INPUT {
                return val;
            }
            visited.insert( (px, y), val );
        }
        py = -py;
        println!( "Right ring {}", ring );
        for x in (px + 1)..(-px + 1) {
            let val = sum_neighbors( &mut visited, x, py );
            if val > INPUT {
                return val;
            }
            visited.insert( (x, py), val );
        }
        px = -px + 1;        
    }
    return 0;
}
