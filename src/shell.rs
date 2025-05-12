use std::collections::HashMap;

use crate::{builtins::get_builtins, commands::command::Command};

#[derive(Debug)]
pub struct Shell {
    commands: &'static HashMap<String, Box<dyn Command>>,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            commands: get_builtins(),
        }
    }

    pub fn parse(&self, args: Vec<&str>, command: String) -> Result<String, String> {
        let command = self.commands.get(&command);

        if let Some(command) = &command {
            return command.parse(args);
        }

        Err("".to_string())
    }
}
