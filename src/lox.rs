use std::io::{stdin, stdout, Write};

pub(crate) struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) {
        let contents = std::fs::read_to_string(path);

        match contents {
            Ok(contents) => {
                self.run(contents);

                // Indicate an error in the exit code.
                if self.had_error {
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
                    self.had_error = false;
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

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}
