use std::env;

use super::command::Command;

#[derive(Debug)]
pub struct PwdCommand {}

impl PwdCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for PwdCommand {
    fn parse(&self, _: Vec<&str>) -> Result<String, String> {
        let path = env::current_dir();

        if let Ok(_path) = path {
            return Ok(format!("{}\n", _path.display()));
        }

        Err("".to_string())
    }
}
