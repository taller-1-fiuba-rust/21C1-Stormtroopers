use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command MSET\n";
const CLIENT_ID: &str = "MsetCommmand";
const RESPONSE_COMMAND: &str = "OK\n";

pub struct MsetCommmand {
    id_job: u32,
    logger: Logger<String>,
}

impl MsetCommmand {
    pub fn new(id_job: u32, logger: Logger<String>) -> MsetCommmand {
        MsetCommmand { id_job, logger }
    }
}

impl Loggable for MsetCommmand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for MsetCommmand {
    fn clone(&self) -> MsetCommmand {
        MsetCommmand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for MsetCommmand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_string_db();
        db.mset(args);

        Ok(String::from(RESPONSE_COMMAND))
    }
}
