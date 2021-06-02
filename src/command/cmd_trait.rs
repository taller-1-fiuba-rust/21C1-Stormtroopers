use crate::app_info::AppInfo;
use crate::errors::run_error::RunError;

pub static PING_COMMAND_STR: &str = "ping";
pub static GET_COMMAND_STR: &str = "get";
pub static SET_COMMAND_STR: &str = "set";
pub static PUBSUB_COMMAND_STR: &str = "pubsub";
pub static FLUSHDB_COMMAND_STR: &str = "flushdb";
pub static DBSIZE_COMMAND_STR: &str = "dbsize";

pub trait Command: Send {
    fn run(&self, args: Vec<&str>, app_info: &AppInfo) -> Result<String, RunError>;
}
