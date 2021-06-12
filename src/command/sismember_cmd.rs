use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command SISMEMBER\n";
const CLIENT_ID: &str = "SismemberCommmand";

pub struct SismemberCommmand {
    id_job: u32,
    logger: Logger<String>,
}

impl SismemberCommmand {
    pub fn new(id_job: u32, logger: Logger<String>) -> SismemberCommmand {
        SismemberCommmand { id_job, logger }
    }
}

impl Loggable for SismemberCommmand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for SismemberCommmand {
    fn clone(&self) -> SismemberCommmand {
        SismemberCommmand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for SismemberCommmand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_set_db();

        let mut res = db.sismember(args).to_string();
        res.push('\n');

        Ok(res)
    }
}
