use crate::errors::WithError;
use std::io::{stdin, stdout, Write};

pub(crate) struct Lox {}

impl WithError for Lox {}

impl Lox {
    pub fn new() -> Self {
        Lox {}
    }

    pub fn run_file(&mut self, path: &str) {
        let contents = std::fs::read_to_string(path);

        match contents {
            Ok(contents) => {
                self.run(contents);

                // Indicate an error in the exit code.
                if Self::has_error() {
                    std::process::exit(65);
                }
            }
            Err(e) => {
                println!("Error reading file: {}", e);
                std::process::exit(74);
            }
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            stdout().write(b"> ").unwrap();
            stdout().flush().unwrap();
            let mut buffer = String::new();
            match stdin().read_line(&mut buffer) {
                Ok(_) => {
                    // If the user just pressed enter, we want to exit the prompt.
                    if buffer.trim_end() == "" {
                        break;
                    }
                    // Otherwise, we want to run the code.
                    self.run(buffer.trim_end().to_string());
                    Self::set_error(false);
                }
                Err(e) => {
                    println!("Error reading from stdin: {}", e);
                    std::process::exit(74);
                }
            }
        }
    }

    pub fn run(&mut self, source: String) {
        // Here you need to scans the source code and generate tokens.
        println!("{}", source);
    }
}
