use std::env;
use std::fs;

use rustfuck::exec;

fn main() {
    let mut filename = env::args();
    filename.next();
    let source = fs::read_to_string(filename.next().unwrap()).unwrap();
    let source: String = source.split_whitespace().collect();

    exec(source);
}
