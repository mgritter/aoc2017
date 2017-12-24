const INPUT : &str = "42/37
28/28
29/25
45/8
35/23
49/20
44/4
15/33
14/19
31/44
39/14
25/17
34/34
38/42
8/42
15/28
0/7
49/12
18/36
45/45
28/7
30/43
23/41
0/35
18/9
3/31
20/31
10/40
0/22
1/23
20/47
38/36
15/8
34/32
30/30
30/44
19/28
46/15
34/50
40/20
27/39
3/14
43/45
50/42
1/33
6/39
46/44
22/35
15/20
43/31
23/23
19/27
47/15
43/43
25/36
26/38
1/10";

// Return length, stregnth

use std::cmp::max;

fn dfs_bridge( remaining_components : &Vec<(u32,u32)>,
               last_port : u32 ) -> (u32,u32) {
    let mut best_len = 0;
    let mut best_str = 0;
    
    for i in 0..remaining_components.len() {
        let (a,b) = remaining_components[i];
        if a == last_port || b == last_port {
            let mut c2 = remaining_components.clone();
            c2.swap_remove( i );
            let (len,stren) = dfs_bridge(
                &c2,
                if a == last_port {
                    b
                } else {
                    a
                }
            );
            let my_len = len + 1;
            let my_str = stren + a + b;
            // Part 2: best length, tiebreak by strength
            if my_len > best_len {
                best_len = my_len;
                best_str = my_str;
            } else if my_len == best_len {
                best_str = max( best_str, my_str );
            }
        }
    }
    (best_len, best_str)
}
                                                          
fn main() {
    let mut components : Vec<(u32,u32)> = Vec::new();
    for line in INPUT.split( "\n" ) {
        let tok : Vec<u32> = line.split( "/" ).
            map( |x| x.parse::<u32>().unwrap() ).collect();
        components.push( ( tok[0], tok[1] ) );
    }

    let (best_len, best_str) = dfs_bridge( &components, 0 );
    println!( "Best bridge length: {}", best_len );
    println!( "Best bridge strength: {}", best_str );
}
