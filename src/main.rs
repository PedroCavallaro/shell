mod builtins;
mod commands;
mod shell;

#[allow(unused_imports)]
use std::io::{self, Write};

use shell::Shell;

fn parse_command(input: String) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }
            '\\' if in_double_quotes => {
                if let Some(&next) = chars.peek() {
                    match next {
                        '\\' | '"' | '$' => {
                            current.push(chars.next().unwrap());
                        }
                        'n' => {
                            chars.next();

                            current.push('\n');
                        }
                        _ => {
                            current.push('\\');
                        }
                    }
                } else {
                    current.push('\\');
                }
            }
            ' ' if !in_single_quotes && !in_double_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}

fn main() {
    let client = Shell::new();

    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let command_args = parse_command(input.trim_end().to_string().clone());
        //redirect in future

        let command = command_args[0].clone();
        let args: Vec<String> = command_args[1..].to_vec();

        let result = client.parse(args, command.clone());

        match result {
            Ok(val) => {
                let stdout = val;

                //println!("{}", stdout);

                io::stdout().write_all(stdout.as_bytes()).unwrap()
            }

            Err(err) => {
                if !err.is_empty() {
                    io::stdout().write_all(err.to_string().as_bytes()).unwrap();

                    continue;
                }

                io::stdout()
                    .write_all(
                        format!("{}: command not found\n", command)
                            .to_string()
                            .as_bytes(),
                    )
                    .unwrap();
            }
        };
    }
}
