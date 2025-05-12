mod commands;
mod shell;

#[allow(unused_imports)]
use std::io::{self, Write};

use shell::Shell;

fn main() {
    let client = Shell::new();

    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let command: Vec<&str> = input.trim().split(" ").collect();

        let result = client.parse("".to_string(), command[0].to_string());

        match result {
            Ok(val) => io::stdout().write_all(val.as_bytes()).unwrap(),
            Err(_) => {
                io::stdout()
                    .write_all(
                        format!("{}: command not found\n", command[0])
                            .to_string()
                            .as_bytes(),
                    )
                    .unwrap();
            }
        };
    }
}
