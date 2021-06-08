use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command RENAME\n";
const CLIENT_ID: &str = "RenameCommmand";
const RESPONSE_COMMAND: &str = "OK\n";

pub struct RenameCommmand {
    id_job: u32,
    logger: Logger<String>,
}

impl RenameCommmand {
    pub fn new(id_job: u32, logger: Logger<String>) -> RenameCommmand {
        RenameCommmand { id_job, logger }
    }
}

impl Loggable for RenameCommmand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for RenameCommmand {
    fn clone(&self) -> RenameCommmand {
        RenameCommmand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for RenameCommmand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let mut db = app_info.get_string_db();

        match db.rename(String::from(args[0]), String::from(args[1])) {
            Ok(()) => Ok(String::from(RESPONSE_COMMAND)),
            Err(e) => Err(e),
        }
    }
}
