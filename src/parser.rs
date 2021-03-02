use std::process;

#[derive(Clone)]
pub enum Syntax {
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
pub enum Exec {
    IPtr,
    DPtr,
    Increment,
    Decrement,
    Write,
    Read,
    LoopBody(Vec<Exec>),
}

pub fn lexer(src: &String) -> Vec<Syntax> {
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
            None => (),
        }
    }

    return seq;
}

pub fn parse(source: Vec<Syntax>) -> Vec<Exec> {
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
                }
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
                }
                Syntax::LoopEnd => {
                    loop_count -= 1;
                    if loop_count == 0 {
                        opcode.push(Exec::LoopBody(parse(source[loop_start + 1..i].to_vec())));
                    }
                }
                _ => (),
            }
        }
    }
    return opcode;
}
