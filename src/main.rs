mod builtins;
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

        let args = command[1..].to_vec();

        let result = client.parse(args, command[0].to_string());

        match result {
            Ok(val) => io::stdout().write_all(val.as_bytes()).unwrap(),
            Err(err) => {
                if !err.is_empty() {
                    io::stdout().write_all(err.to_string().as_bytes()).unwrap();

                    continue;
                }

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
