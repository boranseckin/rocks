use std::{env, process};

use rocks_lang::rocks;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rocks = rocks::new();

    println!("rocks v{}", env!("CARGO_PKG_VERSION"));

    match args.len() {
        n if n > 2 => {
            println!("Usage: rocks [script]");
            process::exit(64);
        },
        2 => rocks.run_file(args[1].clone()),
        _ => rocks.run_prompt(),
    };
}
