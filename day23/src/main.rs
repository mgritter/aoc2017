const INPUT : &str = "set b 79
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23";

use std::collections::HashMap;

#[derive(Debug)]
enum Command {
    Set( char, String ),
    Sub( char, String ),
    Mul( char, String ),
    Jnz( char, String )    
}

struct Program<'a> {
    pc : usize,
    program : &'a Vec<Command>,
    registers : HashMap<char,i64>,
    completed : bool,
    num_mult : usize
}

impl<'a> Program<'a> {
    fn create( program : &'a Vec<Command> ) -> Program<'a> {
        let mut p = Program {
            pc : 0,
            program : program,
            registers : HashMap::new(),
            completed : false,
            num_mult : 0
        };
        // Part two
        //p.registers.insert( 'a', 1 );
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
        println!( "{:?} | {} {:?}", self.registers,
                                    self.pc, self.program[self.pc] );

        match self.program[self.pc] {
            Command::Set( reg, ref arg ) => {
                let val = self.parse_argument( arg );
                *(self.get_dst( reg )) = val;
                self.pc += 1;
            }
            Command::Sub( reg, ref arg ) => {
                let val = self.parse_argument( arg );
                *(self.get_dst( reg )) -= val;
                self.pc += 1;
            }
            Command::Mul( reg, ref arg ) => {
                self.num_mult += 1;
                let val = self.parse_argument( arg );
                *(self.get_dst( reg )) *= val;
                self.pc += 1;
            }
            Command::Jnz( reg, ref arg ) => {
                let cond = *(self.get_dst( reg )) != 0;
                let offs = self.parse_argument( arg );
                if cond {
                    let dest =  (self.pc as i64) + offs;
                    if dest < 0 {
                        println!( "Terminating due to PC < 0" );
                        self.completed = true;
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
            self.completed = true;
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
                "set" => Command::Set( reg, tok[2].to_string() ),
                "sub" => Command::Sub( reg, tok[2].to_string() ),
                "mul" => Command::Mul( reg, tok[2].to_string() ),
                "jnz" => Command::Jnz( reg, tok[2].to_string() ),
                _ => { panic!( "Unexpected command." ); } 
            }
        );
    }

    let mut p = Program::create( &program );
    while !p.completed {
        p.step();
    }
    println!( "Multiplications: {}", p.num_mult );
}
