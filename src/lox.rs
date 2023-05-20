use std::io::{stdin, stdout, Write};

pub fn run_file(path: &str) {
    let contents = std::fs::read_to_string(path);

    match contents {
        Ok(contents) => {
            run(contents);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            std::process::exit(74);
        }
    }
}

pub fn run_prompt() {
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
                run(buffer.trim_end().to_string());
            }
            Err(e) => {
                println!("Error reading from stdin: {}", e);
                std::process::exit(74);
            }
        }
    }
}

fn run(source: String) {
    // Here you need to scans the source code and generate tokens.
    println!("{}", source);
}