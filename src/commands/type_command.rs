use std::os::unix::fs::PermissionsExt;
use std::{env, ffi::OsString, fs, path::PathBuf};

use crate::builtins::get_builtins;

use super::command::Command;

#[derive(Debug)]
pub(crate) struct TypeCommand {}

impl TypeCommand {
    pub fn new() -> Self {
        Self {}
    }

    fn is_executable(path: &PathBuf) -> bool {
        match fs::metadata(path) {
            Ok(metadata) => {
                let execution_mask = 0o111;

                metadata.permissions().mode() & execution_mask != 0
            }
            Err(_) => false,
        }
    }

    fn find_path(paths: OsString, command: String) -> Option<String> {
        for path in env::split_paths(&paths) {
            let full_path = PathBuf::from(&path).join(command.clone());

            if full_path.exists() && Self::is_executable(&full_path) {
                return Some(format!("{} is {}\n", command, full_path.to_string_lossy()));
            }
        }

        None
    }
}

impl Command for TypeCommand {
    fn parse(&self, args: Vec<String>) -> Result<String, String> {
        let builtins = get_builtins();

        let paths = env::var_os("PATH");

        if paths.is_none() {
            return Err(format!("{}: not found\n", args[0]));
        }

        let command = args[0].clone();

        let command_path = Self::find_path(paths.unwrap(), command.clone());

        if let Some(_built_in) = builtins.get(&command) {
            return Ok(format!("{} is a shell builtin\n", command));
        }

        if let Some(_command_path) = command_path {
            return Ok(_command_path);
        }

        Err(format!("{}: not found\n", args[0]))
    }
}
