use std::env;

mod errors;
mod lox;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = lox::Lox::new();

    if args.len() == 2 {
        interpreter.run_file(&args[1]);
        return;
    }

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    }

    interpreter.run_prompt();
}
