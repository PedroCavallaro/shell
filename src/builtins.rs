use std::collections::HashMap;
use std::sync::OnceLock;

use crate::commands::{
    cd::CdCommand, command::Command, echo::EchoCommand, exit::ExitCommand, pwd::PwdCommand,
    type_command::TypeCommand,
};

type BoxCommand = Box<dyn Command>;

fn build_command(name: &str, command: impl Command + 'static) -> (String, BoxCommand) {
    (name.to_string(), Box::new(command))
}

pub static BUILT_INS: OnceLock<HashMap<String, BoxCommand>> = OnceLock::new();

pub fn get_builtins() -> &'static HashMap<String, BoxCommand> {
    BUILT_INS.get_or_init(|| {
        HashMap::from([
            build_command("exit", ExitCommand::new()),
            build_command("echo", EchoCommand::new()),
            build_command("type", TypeCommand::new()),
            build_command("pwd", PwdCommand::new()),
            build_command("cd", CdCommand::new()),
        ])
    })
}
