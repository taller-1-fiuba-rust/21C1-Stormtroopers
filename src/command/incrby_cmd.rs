use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};
use crate::LINE_BREAK;

const INFO_DBSIZE_COMMAND: &str = "Run command DECRBY\n";
const CLIENT_ID: &str = "DecrbyCommmand";

pub struct IncrbyCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl IncrbyCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> IncrbyCommand {
        IncrbyCommand { id_job, logger }
    }
}

impl Loggable for IncrbyCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for IncrbyCommand {
    fn clone(&self) -> IncrbyCommand {
        IncrbyCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for IncrbyCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_DBSIZE_COMMAND);
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_string_db();

        let rsp = db.incrby(args[0].to_string(), args[1].to_string())?;
        let mut response = rsp.to_string();
        response.push(LINE_BREAK);
        Ok(response)
    }
}
