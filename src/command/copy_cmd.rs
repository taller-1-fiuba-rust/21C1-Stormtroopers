use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command COPY\n";
const CLIENT_ID: &str = "CopyCommmand";

pub struct CopyCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl CopyCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> CopyCommand {
        CopyCommand { id_job, logger }
    }
}

impl Loggable for CopyCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for CopyCommand {
    fn clone(&self) -> CopyCommand {
        CopyCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for CopyCommand {
    fn run(&self, args: Vec<&str>, app_info: &AppInfo) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let mut structure = app_info.get_structure();
        let resp = structure.copy(String::from(args[0]), String::from(args[1]));
        let mut resp_str = resp.to_string();
        resp_str.push('\n');
        Ok(resp_str)
    }
}
