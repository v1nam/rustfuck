use std::io;
use crate::parser::Exec;


pub fn run(instructs: &Vec<Exec>, system: &mut Vec<u8>, cell: &mut usize) -> Vec<char> {
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
