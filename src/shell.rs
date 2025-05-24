use core::str;
use std::{collections::HashMap, process::Command as Exec};

use crate::{
    builtins::get_builtins,
    commands::{command::Command, type_command::TypeCommand},
};

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

    fn is_exec_program(command: &str) -> bool {
        let type_command = TypeCommand::new();

        let path = type_command.parse([command.to_string()].to_vec());

        if let Ok(_path) = path {
            return true;
        }

        false
    }

    fn exec_program(command: &String, args: &[String]) -> bool {
        let is_exec = Self::is_exec_program(command);

        if !is_exec {
            return false;
        }

        let child = Exec::new(command).args(args).spawn();

        if let Ok(mut _child) = child {
            let _ = _child.wait();
        }

        true
    }

    pub fn parse(&self, args: Vec<String>, command: String) -> Result<String, String> {
        let builtin_command = self.commands.get(&command);

        if builtin_command.is_none() {
            let quit = Self::exec_program(&command, &args);

            if quit {
                return Ok(String::from(""));
            }
        }

        if let Some(builtin_command) = &builtin_command {
            return builtin_command.parse(args);
        }

        Err("".to_string())
    }
}
