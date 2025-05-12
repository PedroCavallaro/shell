use super::command::Command;

#[derive(Debug)]
pub struct EchoCommand {}

impl EchoCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for EchoCommand {
    fn parse(&self, args: String) -> Result<String, ()> {
        Ok("".to_string())
    }
}
