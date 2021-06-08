use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command EXISTS\n";
const CLIENT_ID: &str = "ExistsCommmand";

pub struct ExistsCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl ExistsCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> ExistsCommand {
        ExistsCommand { id_job, logger }
    }
}

impl Loggable for ExistsCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for ExistsCommand {
    fn clone(&self) -> ExistsCommand {
        ExistsCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for ExistsCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_string_db();

        let mut result_del = db.exists(args).to_string();
        result_del.push('\n');

        Ok(result_del)
    }
}
