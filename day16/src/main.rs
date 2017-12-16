#[macro_use]
extern crate nom;
use nom::{digit,alpha};
use std::str;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum Command {
    Spin( usize ),
    Exchange( usize, usize ),
    Partner( char, char )
}

named!(position<usize>, do_parse!(
    numerals: digit >>
    ( str::from_utf8( numerals ).unwrap().parse::<usize>().unwrap() )
));

named!( parse_spin<Command>, do_parse!(
    tag!( "s" ) >>
        x : position >>
        ws!(eof!()) >>
    ( Command::Spin( x ) )
));    

named!( parse_exchange<Command>, do_parse!(
    tag!( "x" ) >>
        x : position >>
        tag!( "/" ) >>
        y : position >> 
        ws!(eof!()) >>
    ( Command::Exchange( x, y ) )
));    

named!( parse_partner<Command>, do_parse!(
    tag!( "p" ) >>
        x : alpha >>
        tag!( "/" ) >>
        y : alpha >> 
        ws!(eof!()) >>
    ( Command::Partner( x[0] as char, y[0] as char ) )
));

named!( parse_command<Command>, alt!(
    parse_partner |
    parse_exchange |
    parse_spin
));

fn main() {
    let initial_v : Vec<char> = "abcdefghijklmnop".chars().collect();
    let mut v = initial_v.clone();
    
    let mut commands : Vec<Command> = Vec::new();
    
    let mut file = File::open( "src/input.txt" ).expect( "Couldn't open src/input.txt" );
    let mut commands_txt = String::new();
    file.read_to_string( &mut commands_txt ).expect( "Couldn't read file." );
    
    for dance in commands_txt.split( ',' ) {
        let (_, command) = parse_command( dance.as_bytes() ).unwrap();
        commands.push( command );
    }

    let mut iter = 0;
    let mut visited : Vec<String> = Vec::new();
    visited.push( initial_v.iter().collect() );
                  
    while iter < 1000000000 {        
        for command in commands.iter() {
            match *command {
                Command::Exchange( x, y ) => {
                    let tmp = v[x];
                    v[x] = v[y];
                    v[y] = tmp;
                }
                Command::Partner( a, b ) => {
                    let x = v.iter().position( |&i| i == a ).unwrap();
                    let y = v.iter().position( |&i| i == b ).unwrap();
                    let tmp = v[x];
                    v[x] = v[y];
                    v[y] = tmp;
                    
                }
                Command::Spin( x ) => {                
                    // FIXME: horribly inefficient.
                    for _ in 0..x {
                        let c = v.pop().unwrap();
                        v.insert( 0, c );
                    }
                }
            }
        }
        
        let p : String = v.iter().collect();
        iter += 1;

        if visited[0] == p {
            println!( "Iteration {} matches 0", iter );
            let fin = 1000000000 % iter;
            println!( "Final position: {}", visited[fin] );
            break;
        } else {
            visited.push( p );
        }
    }
}
