use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};

const INFO_EXPIREAT_COMMAND: &str = "Run command EXPIRE_AT\n";
const CLIENT_ID: &str = "ExpireAtCommmand";

pub struct ExpireAtCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl ExpireAtCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> ExpireAtCommand {
        ExpireAtCommand { id_job, logger }
    }
}

impl Loggable for ExpireAtCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for ExpireAtCommand {
    fn clone(&self) -> ExpireAtCommand {
        ExpireAtCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for ExpireAtCommand {
    fn run(&self, args: Vec<&str>, app_info: &AppInfo) -> Result<String, RunError> {
        let _log_info_res = self.logger.info(self, INFO_EXPIREAT_COMMAND);

        
        
    }
}
