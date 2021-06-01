use crate::errors::run_error::RunError;
use crate::structure_string::StructureString;

use std::sync::Arc;

pub static PING_COMMAND_STR: &str = "ping";
pub static GET_COMMAND_STR: &str = "get";
pub static SET_COMMAND_STR: &str = "set";

pub trait Command: Send {
    fn run(
        &self,
        args: Vec<&str>,
        structure: Arc<StructureString<String>>,
    ) -> Result<String, RunError>;
}
