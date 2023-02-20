use std::{env, process};

use rlox::rlox;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rlox = rlox::new();

    match args.len() {
        n if n > 2 => {
            println!("Usage: rlox [script]");
            process::exit(64);
        },
        2 => rlox.run_file(args[1].clone()),
        _ => rlox.run_prompt(),
    };
}

