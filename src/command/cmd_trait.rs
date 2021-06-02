use crate::app_info::AppInfo;
use crate::errors::run_error::RunError;

pub static PING_COMMAND_STR: &str = "ping";
pub static GET_COMMAND_STR: &str = "get";
pub static SET_COMMAND_STR: &str = "set";
pub static PUBSUB_COMMAND_STR: &str = "pubsub";
pub static FLUSHDB_COMMAND_STR: &str = "flushdb";
pub static DBSIZE_COMMAND_STR: &str = "dbsize";

pub trait Command: Send + CommandClone {
    fn run(&self, args: Vec<&str>, app_info: &AppInfo) -> Result<String, RunError>;
}

pub trait CommandClone {
    fn clone_box(&self) -> Box<dyn Command>;
}

impl<T> CommandClone for T where T: 'static + Command + Clone, {
    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Box<dyn Command> {
        self.clone_box()
    }
}