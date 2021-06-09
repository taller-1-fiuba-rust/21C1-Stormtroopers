use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command RPUSH\n";
const CLIENT_ID: &str = "RPushCommmand";

pub struct RPushCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl RPushCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> RPushCommand {
        RPushCommand { id_job, logger }
    }
}

impl Loggable for RPushCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for RPushCommand {
    fn clone(&self) -> RPushCommand {
        RPushCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for RPushCommand {
    fn run(
        &self,
        mut args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_list_db();

        let key = args[0].to_string();
        args.remove(0);

        let mut result = db.rpush(key, args)?;
        result.push('\n');

        Ok(result)
    }
}
