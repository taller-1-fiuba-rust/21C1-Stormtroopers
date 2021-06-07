use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};

const INFO_RUN_COMMAND: &str = "Run command GET\n";
const CLIENT_ID: &str = "SetCommand";

pub struct GetCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl GetCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> GetCommand {
        GetCommand { id_job, logger }
    }
}

impl Loggable for GetCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for GetCommand {
    fn clone(&self) -> GetCommand {
        GetCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for GetCommand {
    fn run(&self, args: Vec<&str>, app_info: &AppInfo) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_RUN_COMMAND);
        if let Ok(_r) = log_info_res {}

        let structure = app_info.get_structure();
        let mut string = structure.get_string(String::from(args[0]));
        string.push('\n');

        Ok(string)
    }
}
