use crate::errors::run_error::RunError;

pub trait Command: Send {
    fn run(&self, args: Vec<&str>) -> Result<String, RunError>;
}