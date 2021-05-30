use crate::errors::run_error::RunError;

pub trait Command {
    fn run(&self, args: &str) -> Result<String, RunError>;
}