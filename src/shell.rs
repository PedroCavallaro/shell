use std::collections::HashMap;

use crate::commands::{command::Command, echo::EchoCommand, exit::ExitCommand};

#[derive(Debug)]
pub struct Shell {
    commands: HashMap<String, Box<dyn Command>>,
}

type BoxCommand = Box<dyn Command>;

impl Shell {
    fn build_command(name: &str, command: impl Command + 'static) -> (String, BoxCommand) {
        (name.to_string(), Box::new(command))
    }

    fn get_commands() -> HashMap<String, Box<dyn Command>> {
        HashMap::from([
            Self::build_command("exit", ExitCommand::new()),
            Self::build_command("echo", EchoCommand::new()),
        ])
    }

    pub fn new() -> Self {
        let commands = Self::get_commands();

        Shell { commands }
    }

    pub fn parse(&self, args: String, command: String) -> Result<String, ()> {
        let command = self.commands.get(&command);

        if let Some(command) = command {
            return command.parse(args);
        }

        Err(())
    }
}
