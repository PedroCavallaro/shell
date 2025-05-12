use super::command::Command;

#[derive(Debug)]
pub struct EchoCommand {}

impl EchoCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for EchoCommand {
    fn parse(&self, args: Vec<&str>) -> Result<String, String> {
        let output = args.join(" ");

        Ok(format!("{}\n", output))
    }
}
