use crate::errors::run_error::RunError;

pub trait Validate {
    fn validate_args(&self, args: Vec<&str>) -> Result<bool, RunError>;
}
