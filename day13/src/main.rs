const INPUT : &str = "0: 3
1: 2
2: 5
4: 4
6: 4
8: 6
10: 6
12: 6
14: 8
16: 6
18: 8
20: 8
22: 8
24: 12
26: 8
28: 12
30: 8
32: 12
34: 12
36: 14
38: 10
40: 12
42: 14
44: 10
46: 14
48: 12
50: 14
52: 12
54: 9
56: 14
58: 12
60: 12
64: 14
66: 12
70: 14
76: 20
78: 17
80: 14
84: 14
86: 14
88: 18
90: 20
92: 14
98: 18";

const SAMPLE : &str = "0: 3
1: 2
4: 4
6: 4";

fn gcd( a : u64, b : u64 ) -> u64 {
    if b > a {
        return gcd( b, a );
    }
    let r = a % b;
    if r == 0 {
        return b;
    } else {
        return gcd( b, r );
    }
}

fn lcm( a : u64, b : u64 ) -> u64 {
    return a * b / gcd( a, b );
}

fn main() {
    let mut ranges : Vec<u32> = Vec::new();
    let mut position : Vec<u32> = Vec::new();
    let mut direction : Vec<i32> = Vec::new();
    
    for line in INPUT.split( "\n" ) {
        let token : Vec<u32> = line.split( ": ").map( |x| x.parse().unwrap() ).collect();
        let layer = token[0] as usize;
        let range = token[1];
        while ranges.len() < layer + 1 {
            ranges.push( 0 );
            position.push( 0 );
            direction.push( 1 );
        }
        ranges[layer] = range;
    }
    let num_layers = ranges.len();
    println!( "Layers: 0 through {}", num_layers - 1 );
    
    let mut possible : Vec<u64> = Vec::new();
    let mut m = 2;
    possible.push( 0 );
    possible.push( 1 );
    
    for layer in 0..num_layers {
        if ranges[layer] != 0 {
            // Visit each twice per cycle, except for the ends where
            // we turn around.
            let cycle_length = ( 2*ranges[layer] - 2 ) as u64;
            let forbidden = ( cycle_length - ( layer as u64 % cycle_length ) ) % cycle_length;
                
            println!( "x != {} mod {}", forbidden, cycle_length );
            let m_prime = lcm( cycle_length, m );
            if m_prime != m {
                let mut new_possible : Vec<u64> = Vec::new();
                for k in possible.iter() {
                    for j in 0..(m_prime/m) {
                        // Say we knew only 1 mod 2 was possible before
                        // and now the LCM is 14, now the possible values
                        // 1 3 5 7 9 11 13.
                        new_possible.push( k + j * m )
                    }
                }
                println!( "M={} expanded={:?}", m_prime, new_possible );
                possible = new_possible;
                m = m_prime;
            }
            // Filter to only those not obeying the inequality above
            let mut new_possible : Vec<u64> = Vec::new();
            for k in possible.iter() {
                if k % cycle_length != forbidden {
                    new_possible.push( *k );
                }
            }
            println!( "filtered={:?}", new_possible );
            possible = new_possible;
        }        
    }

    possible.insert( 0, 0 );
    possible.sort();
    
    for di in possible.iter() {
        let delay = *di;
        println!( "Trying delay {}", delay );
        let mut severity = 0;
        let mut caught = false;
        
        for i in 0..num_layers {
            direction[i] = 1;
            position[i] = 0;
        }
                   
        for pico in 0..(num_layers as u64 + delay) {
            // Entering layer 0 at time delay
            if pico >= delay {
                let entering = (pico - delay) as usize;
                // println!( "Entering layer {} at time {}",
                //          entering, pico );
                
                if ranges[entering] != 0 {
                    // Firewall here
                    if position[entering] == 0 {
                        // Caught entering
                        severity += entering * ranges[entering] as usize;
                        caught = true;
                        println!( "Caught at layer {}, severity now {}",
                                   entering, severity );
                    }
                }
            }
            
            for i in 0..num_layers {
                if ranges[i] != 0 {
                    if direction[i] == 1 {
                        position[i] = position[i] + 1;
                        if position[i] == ranges[i] - 1 {
                            direction[i] = -1;
                        }
                    } else {
                        position[i] = position[i] - 1;
                        if position[i] == 0 {
                            direction[i] = 1;
                        }
                    }
                }
            }
        }

        if !caught {
            println!( "Undetected!" );
            break;
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn gcd_test() {
        assert_eq!( gcd( 6, 3 ), 3 );
        assert_eq!( gcd( 3, 3 ), 3 );
        assert_eq!( gcd( 3, 6 ), 3 );
        assert_eq!( gcd( 12, 6 ), 6 );
        assert_eq!( gcd( 12, 2 ), 2 );
        assert_eq!( gcd( 15, 13 ), 1 );
    }
}
