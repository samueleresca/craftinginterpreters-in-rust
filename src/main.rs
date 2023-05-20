use std::env;

use lox::{run_file, run_prompt};

mod lox;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        run_file(&args[1]);
        return;
    }

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    }

    run_prompt();
}