use std::env;

use super::command::Command;

#[derive(Debug)]
pub struct CdCommand {}

impl CdCommand {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_path(path: String) -> String {
        if !path.contains('~') {
            return path.to_string();
        }

        let home = env::var("HOME");

        if let Ok(home_dir) = home {
            return path.replacen('~', &home_dir, 1);
        }

        path.to_string()
    }
}

impl Command for CdCommand {
    fn parse(&self, args: Vec<String>) -> Result<String, String> {
        if args.is_empty() {
            return Ok("".to_string());
        }

        let path = args[0].clone();

        let dir_path = CdCommand::get_path(path);

        if env::set_current_dir(&dir_path).is_err() {
            return Err(format!("cd: {}: No such file or directory\n", dir_path));
        }

        Ok("".to_string())
    }
}
