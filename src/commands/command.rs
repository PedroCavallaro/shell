use std::fmt::Debug;

pub trait Command: Debug {
    fn parse(&self, args: String) -> Result<String, ()>;
}
