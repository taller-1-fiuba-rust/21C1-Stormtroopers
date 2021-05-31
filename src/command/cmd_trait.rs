use crate::errors::run_error::RunError;
use crate::structure_string2::StructureString;
use crate::structure_simple::StructureSimple;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub static PING_COMMAND_STR: &str = "ping";
pub static GET_COMMAND_STR: &str = "get";
pub static SET_COMMAND_STR: &str = "set";

pub trait Command: Send {
    fn run(&self, args: Vec<&str>, structure: & Arc<Mutex<HashMap<String,String>>>) -> Result<String, RunError>;
}