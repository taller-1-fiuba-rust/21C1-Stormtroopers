use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command SADD\n";
const CLIENT_ID: &str = "SAddCommmand";

pub struct SAddCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl SAddCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> SAddCommand {
        SAddCommand { id_job, logger }
    }
}

impl Loggable for SAddCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for SAddCommand {
    fn clone(&self) -> SAddCommand {
        SAddCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for SAddCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_set_db();

        let mut result = db.sadd(args).to_string();
        result.push('\n');

        Ok(result)
    }
}
