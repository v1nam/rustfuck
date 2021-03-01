use std::env;
use std::fs;
use std::io;
use std::process;


#[derive(Clone)]
enum Syntax {
    IPtr,
    DPtr,
    Increment,
    Decrement,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}


#[derive(Clone)]
enum Exec {
    IPtr,
    DPtr,
    Increment,
    Decrement,
    Write,
    Read,
    LoopBody(Vec<Exec>),
}


fn lexer(src: &String) -> Vec<Syntax> {
    let mut seq: Vec<Syntax> = Vec::new();
    for ch in src.chars() {
        let oper = match ch {
            '>' => Some(Syntax::IPtr),
            '<' => Some(Syntax::DPtr),
            '+' => Some(Syntax::Increment),
            '-' => Some(Syntax::Decrement),
            '.' => Some(Syntax::Write),
            ',' => Some(Syntax::Read),
            '[' => Some(Syntax::LoopBegin),
            ']' => Some(Syntax::LoopEnd),
            _ => None,
        };

        match oper {
            Some(s) => seq.push(s),
            None => ()
        }
    }

    return seq;
}


fn parse(source: Vec<Syntax>) -> Vec<Exec> {
    let mut opcode: Vec<Exec> = Vec::new();
    let mut loop_start = 0;
    let mut loop_count = 0;

    for (i, ins) in source.iter().enumerate() {
        if loop_count == 0 {
            let op = match ins {
                Syntax::IPtr => Some(Exec::IPtr),
                Syntax::DPtr => Some(Exec::DPtr),
                Syntax::Increment => Some(Exec::Increment),
                Syntax::Decrement => Some(Exec::Decrement),
                Syntax::Write => Some(Exec::Write),
                Syntax::Read => Some(Exec::Read),
                Syntax::LoopBegin => {
                    loop_start = i;
                    loop_count += 1;
                    None
                },
                Syntax::LoopEnd => process::exit(1),
            };

            match op {
                Some(s) => opcode.push(s),
                None => (),
            }
        } else {
            match ins {
                Syntax::LoopBegin => {
                    loop_count += 1;
                },
                Syntax::LoopEnd => {
                    loop_count -= 1;
                    if loop_count == 0 {
                        opcode.push(Exec::LoopBody(parse(source[loop_start+1..i].to_vec())));
                    }
                },
                _ => (),
            }
        }
    }
    return opcode;
}

fn run(instructs: &Vec<Exec>, system: &mut Vec<u8>, cell: &mut usize) -> Vec<char> {
    let mut stdout: Vec<char> = Vec::new();
    for sym_i in 0..instructs.len() {
        match &instructs[sym_i] {
            Exec::IPtr => {
                *cell += 1;
                if system.len()-1 <= *cell {system.push(0);} 
            },

            Exec::DPtr => {
                *cell -= 1;
            },

            Exec::Increment => system[*cell] += 1,
            Exec::Decrement => system[*cell] -= 1,
            Exec::Write => {
                stdout.push(system[*cell] as char);
            },
            
            Exec::Read => {
                let mut raw_inp = String::new();
                io::stdin().read_line(&mut raw_inp).expect("failed to read input");
                system[*cell] = raw_inp.parse::<u8>().unwrap();
            },
            Exec::LoopBody(lp) => {
                while system[*cell] != 0 {
                    run(&lp, system, cell);
                }
            }
        }
    }
    return stdout;
}

fn main() {
    let mut filename = env::args();
    filename.next();
    let source = fs::read_to_string(filename.next().unwrap()).unwrap();
    let source: String = source.split_whitespace().collect();
    let mut system = vec![0];
    let mut cell = 0;
    println!("{}", run(&parse(lexer(&source)), &mut system, &mut cell).iter().collect::<String>());
}
