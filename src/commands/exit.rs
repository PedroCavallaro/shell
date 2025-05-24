use super::command::Command;

#[derive(Debug)]
pub struct ExitCommand {}

impl ExitCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for ExitCommand {
    fn parse(&self, _: Vec<String>) -> Result<String, String> {
        std::process::exit(0);
    }
}
