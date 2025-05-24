use std::fmt::Debug;

pub(crate) trait Command: Sync + Send + Debug {
    fn parse(&self, args: Vec<String>) -> Result<String, String>;
}

#[allow(unused)]
pub(crate) enum CommandError {
    NotFound,
    Invalid,
}
