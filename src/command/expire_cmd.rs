use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};

const INFO_EXPIRE_COMMAND: &str = "Run command EXPIRE\n";
const CLIENT_ID: &str = "ExpireCommmand";

pub struct ExpireCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl ExpireCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> ExpireCommand {
        ExpireCommand { id_job, logger }
    }
}

impl Loggable for ExpireCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for ExpireCommand {
    fn clone(&self) -> ExpireCommand {
        ExpireCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for ExpireCommand {
    fn run(&self, args: Vec<&str>, app_info: &AppInfo) -> Result<String, RunError> {
        let _log_info_res = self.logger.info(self, INFO_EXPIRE_COMMAND);

        Ok(String::from("EXPIRE command ok"))
    }
}
