fn main() {
    // (generator A uses 16807; generator B uses 48271)
    // Generator A starts with 116
    // Generator B starts with 299
    let mut a : u64 = 116;
    let a_mult = 16807;
    let mut b : u64 = 299;
    let b_mult = 48271;

    let mut count = 0;
    let num_trials = 5000000; // 40000000
    for _ in 0..num_trials {
        while {
            a = (a * a_mult) % 2147483647;
            a % 4 != 0
        } {}

        while {
            b = (b * b_mult) % 2147483647;
            b % 8 != 0
        } {}
        
        // println!( "a = {} b = {}", a, b );
        // lower 16 bits
        if a & 0xffff == b & 0xffff {
            count += 1;
        }
    }
    
    println!( "Count = {}", count );
}
