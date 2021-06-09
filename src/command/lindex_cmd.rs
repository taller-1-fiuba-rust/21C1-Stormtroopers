use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command LINDEX\n";
const CLIENT_ID: &str = "LindexCommmand";

pub struct LindexCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl LindexCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> LindexCommand {
        LindexCommand { id_job, logger }
    }
}

impl Loggable for LindexCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LindexCommand {
    fn clone(&self) -> LindexCommand {
        LindexCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LindexCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_list_db();

        let mut result = db.lindex(args[0].to_string(), args[1].to_string())?;
        result.push('\n');

        Ok(result)
    }
}
