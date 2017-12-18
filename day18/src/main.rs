const INPUT : &str = "set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 680
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19";

#[derive(Debug)]
enum Command {
    Snd( char ),
    Set( char, String ),
    Add( char, String ),
    Mul( char, String ),
    Mod( char, String ),
    Rcv( char ),
    Jgz( char, String )    
}

use std::collections::HashMap;

#[derive(Debug,PartialEq)]
enum ProgramState {
    Runnable,
    BlockedOnReceive,
    Terminated
}

struct Program<'a> {
    id : i64,
    pc : usize,
    program : &'a Vec<Command>,
    registers : HashMap<char,i64>,
    state: ProgramState,
    num_sends : i64,
    channel_in: Vec<i64>,
    channel_out: Vec<i64>
}

impl<'a> Program<'a> {
    fn create( id : i64,
               program : &'a Vec<Command> ) -> Program<'a> {
        let mut p = Program {
            id : id,
            pc : 0,
            program : program,
            registers : HashMap::new(),
            state : ProgramState::Runnable,
            num_sends : 0,
            channel_in : Vec::new(),
            channel_out: Vec::new()
        };
        p.registers.insert( 'p', id );
        p
    }
    
    fn parse_argument( &mut self,
                        arg : &String ) -> i64 {
        let first_char = arg.chars().nth( 0 ).unwrap();
        match arg.parse::<i64>() {
            Ok( v ) => v,
            Err( _ ) => *(self.registers.entry( first_char ).or_insert( 0 ))
        }
    }

    fn get_dst( &mut self,
                 reg : char ) -> &mut i64 {
        // Hack
        if reg == '1' {
            self.registers.entry( reg ).or_insert( 1 )
        } else {       
            self.registers.entry( reg ).or_insert( 0 )
        }
    }
        
    fn step( &mut self ) {
        if self.state == ProgramState::Terminated {
            return;
        }

        if self.state == ProgramState::BlockedOnReceive {
            return;
        }

        println!( "{} {:?} | {} {:?}", self.id, self.registers,
                                       self.pc, self.program[self.pc] );

        match self.program[self.pc] {
            Command::Snd( reg ) => {
                let val = *(self.get_dst( reg ));
                self.channel_out.push( val );
                self.num_sends += 1;
                self.pc += 1;
            }
            Command::Set( reg, ref arg ) => {
                let val = self.parse_argument( arg );
                *(self.get_dst( reg )) = val;
                self.pc += 1;
            }
            Command::Add( reg, ref arg ) => {
                let val = self.parse_argument( arg );
                *(self.get_dst( reg )) += val;
                self.pc += 1;
            }
            Command::Mul( reg, ref arg ) => {
                let val = self.parse_argument( arg );
                *(self.get_dst( reg )) *= val;
                self.pc += 1;
            }
            Command::Mod( reg, ref arg ) => {
                let val = self.parse_argument( arg );
                *(self.get_dst( reg )) %= val;
                self.pc += 1;
            }
            Command::Rcv( reg ) => {
                if self.channel_in.len() > 0 {
                    *(self.get_dst( reg )) = self.channel_in.remove( 0 );
                    self.pc += 1;
                } else {
                    self.state = ProgramState::BlockedOnReceive;
                    return;
                }
            }
            Command::Jgz( reg, ref arg ) => {
                let cond = *(self.get_dst( reg )) > 0;
                let offs = self.parse_argument( arg );
                if cond {
                    let dest =  (self.pc as i64) + offs;
                    if dest < 0 {
                        println!( "Terminating due to PC < 0" );
                        self.state = ProgramState::Terminated;
                    } else {
                        self.pc = dest as usize;
                    }
                } else {
                    self.pc += 1;
                }
            }
        }
        if self.pc >= self.program.len() {
            println!( "Terminating due to PC off end" );
            self.state = ProgramState::Terminated;
        }
    }
}

fn main() {
    let mut program : Vec<Command> = Vec::new();
    for line in INPUT.split( "\n" ) {
        let tok : Vec<&str> = line.split( " ").collect();
        let reg = tok[1].chars().nth( 0 ).unwrap();
        program.push( 
            match tok[0] {
                "snd" => Command::Snd( reg ),
                "set" => Command::Set( reg, tok[2].to_string() ),
                "add" => Command::Add( reg, tok[2].to_string() ),
                "mul" => Command::Mul( reg, tok[2].to_string() ),
                "mod" => Command::Mod( reg, tok[2].to_string() ),
                "rcv" => Command::Rcv( reg ),
                "jgz" => Command::Jgz( reg, tok[2].to_string() ),
                _ => { panic!( "Unexpected command." ); } 
            }
        );
    }

    let mut p = vec!( Program::create( 0, &program ),
                      Program::create( 1, &program ) );

    loop {
        use ProgramState::{Runnable,Terminated,BlockedOnReceive};
        
        if p[0].state == Terminated && p[1].state == Terminated {
            println!( "Both terminated!" );
            break;
        }
        if p[0].state == BlockedOnReceive && p[1].state == BlockedOnReceive {
            println!( "Deadlock!" );
            break;
        }
        let id = {
            if p[0].state == Runnable {
                0
            } else {
                1
            }
        };
        let other_id = 1 - id;

        p[id].step();
        if p[id].channel_out.len() > 0 {
            let v = p[id].channel_out.pop().unwrap();
            p[other_id].channel_in.push( v );
            if p[other_id].state == BlockedOnReceive {
                p[other_id].state = Runnable;
            }
        }
    }
    println!( "Program 1 sends: {}", p[1].num_sends );
                      
                      
 }
