use crate::errors::run_error::RunError;
use crate::structure_string::StructureString;

pub static PING_COMMAND_STR: &str = "ping";
pub static GET_COMMAND_STR: &str = "get";
pub static SET_COMMAND_STR: &str = "set";

pub trait Command: Send {
    fn run(&self, args: Vec<&str>, structure: &mut Box<StructureString<String>>) -> Result<String, RunError>;
}