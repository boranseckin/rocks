use std::{env, process};

use rlox::{run_file, run_prompt};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        n if n > 2 => {
            println!("Usage: rlox [script]");
            process::exit(64);
        },
        2 => run_file(args[1].clone()),
        _ => run_prompt(),
    };
}

