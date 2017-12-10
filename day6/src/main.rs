const INPUT : &str = "0	5	10	0	11	14	13	4	11	8	8	7	1	4	12	11";

use std::collections::HashMap;
use std::cmp::Ord;

fn max_index<T : Ord >( arg: &Vec<T> ) -> Option<usize> {
    // OK, max_by returns the latest of equal elements, and
    // we want the first.
    let mi = arg.iter().enumerate().rev().max_by_key( |&(_i,v)| v );
    mi.and_then( |(i,_v)| Some(i) )
}

fn sow( array : & mut Vec<u32>,
        start : usize ) {
    let num = array[start];
    array[start] = 0;
    for i in 0..num {        
        let pos = ( start + 1 + i as usize ) % array.len();
        array[pos] += 1;
    }    
}

fn main() {
    let nums = INPUT.split( '\t' );
    let mut array : Vec<u32> = nums.map( |x| x.parse().unwrap() ).collect();
    println!( "array: {:?}", array );

    let mut visited : HashMap<Vec<u32>,u32> = HashMap::new();
    let mut num_iterations = 0;

    while !visited.contains_key( &array ) {
        visited.insert( array.clone(), num_iterations );

        let start = max_index( &array ).unwrap();
        println!( "iteration: {}, start: {}", num_iterations, start );
        sow( & mut array, start );        
        println!( "array: {:?}", array );
        num_iterations += 1;
    }

    let first_iteration = visited.get( &array ).unwrap();
    let cycle_length = num_iterations - first_iteration;
    
    println!( "num iterations: {}", num_iterations );
    println!( "cycle length: {}", cycle_length );
}
