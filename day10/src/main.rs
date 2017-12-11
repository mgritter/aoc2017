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
    //let input = vec!( 3, 4, 1, 5 );
    //let mut list : Vec<u32> = (0..5).collect();
    let input = vec!(230,1,2,221,97,252,168,169,57,99,0,254,181,255,235,167);
    let mut list : Vec<u32> = (0..256).collect();
    let mut pos : usize = 0;
    let mut skip_size : usize = 0;
    
    for length in input {
        twist( &mut list, pos, length );
        println!( "Twist: {} {:?}", length, list );
        pos = (pos + length + skip_size) % list.len();
        skip_size += 1;
        println!( "Pos: {}, skip_size: {}", pos, skip_size );
    }
    
    println!( "Final list: {:?}", list );
    println!( "Product: {}", list[0] * list[1] );
}
