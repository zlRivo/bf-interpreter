mod brainfuck;

use brainfuck::*;
use std::env;

fn main() {
    let mut bf = BF::new();
    let args = env::args().collect::<Vec<String>>();
    println!("{:?}", args);
    bf.eval(&args[1]).unwrap();
}
