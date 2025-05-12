use crate::builtins::get_builtins;

use super::command::Command;

#[derive(Debug)]
pub struct TypeCommand {}

impl TypeCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for TypeCommand {
    fn parse(&self, args: Vec<&str>) -> Result<String, String> {
        let builtins = get_builtins();

        if let Some(_command) = builtins.get(args[0]) {
            return Ok(format!("{} is a shell builtin\n", args[0]));
        }

        Err(format!("{}: not found\n", args[0]))
    }
}
