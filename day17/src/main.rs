const INPUT : usize = 376;

fn main() {
    let mut v : Vec<usize> = vec!( 0 );
    let mut cur_pos = 0;

    for i in 1..2018 {
        cur_pos = ( cur_pos + INPUT ) % v.len();
        // Insert *after* that position.
        v.insert( cur_pos + 1, i );
        // the cur_pos is the location of the new value
        cur_pos = cur_pos + 1;
        assert_eq!( v[cur_pos], i );
    }

    let answer_pos = ( cur_pos + 1 ) % v.len();
    println!( "After last insert: {}", v[answer_pos] );

    // Invariant needed for part 2
    assert_eq!( v[0], 0 );

    let mut len = 1;
    let mut zero_follower = 0;
    cur_pos = 0;
    
    for i in 1..50000000 {
        cur_pos = ( cur_pos + INPUT ) % len;
        // Insertion increases length by one
        len += 1;
        // If inserted after zero, record for later.
        if cur_pos == 0 {
            zero_follower = i;
            println!( "Position 0, inserted {}", zero_follower );
        }
        // the cur_pos is the location of the new value
        cur_pos = cur_pos + 1;
    }

    println!( "After zero: {}", zero_follower );            
}
