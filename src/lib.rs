mod execute;
mod parser;

use execute::run;
use parser::{lexer, parse};

pub fn exec(source: String) {
    let mut system = vec![0];
    let mut cell = 0;
    println!(
        "{}",
        run(&parse(lexer(&source)), &mut system, &mut cell)
            .iter()
            .collect::<String>()
    );
}
