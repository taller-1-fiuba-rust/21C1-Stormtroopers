use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command DEL\n";
const CLIENT_ID: &str = "DelCommmand";

pub struct DelCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl DelCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> DelCommand {
        DelCommand { id_job, logger }
    }
}

impl Loggable for DelCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

//TODO: ver por que si vienen varias claves hay que borrar todas
impl Command for DelCommand {
    fn run(&self, _args: Vec<&str>, _app_info: &AppInfo) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}


        Ok(String::from("UNIMPLEMENT COMMAND\n"))
    }
}
