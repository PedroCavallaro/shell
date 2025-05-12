use std::fmt::Debug;

pub trait Command: Sync + Send + Debug {
    fn parse(&self, args: Vec<&str>) -> Result<String, String>;
}
