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

fn main() {
    let mut input : Vec<usize> =
        "230,1,2,221,97,252,168,169,57,99,0,254,181,255,235,167"
        .as_bytes().iter().map( |c| *c as usize ).collect();
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
    
    println!( "Final list: {:?}", list );

    let mut hash : String = String::from( "" );
    for i in 0..16 {
        let hv = list[i*16..(i+1)*16].iter().fold( 0, |acc, &x| acc ^ x );
        hash.push_str( format!( "{:02x}", hv ).as_str() );
    }

    println!( "Hash value: {}", hash );
       
}
