use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command SCARD\n";
const CLIENT_ID: &str = "ScardCommmand";

pub struct ScardCommmand {
    id_job: u32,
    logger: Logger<String>,
}

impl ScardCommmand {
    pub fn new(id_job: u32, logger: Logger<String>) -> ScardCommmand {
        ScardCommmand { id_job, logger }
    }
}

impl Loggable for ScardCommmand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for ScardCommmand {
    fn clone(&self) -> ScardCommmand {
        ScardCommmand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for ScardCommmand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_set_db();

        let mut res = db.scard(args).to_string();
        res.push('\n');

        Ok(res)
    }
}
