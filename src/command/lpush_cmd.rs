use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command LPUSH\n";
const CLIENT_ID: &str = "LpushCommand";

pub struct LpushCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl LpushCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> LpushCommand {
        LpushCommand { id_job, logger }
    }
}

impl Loggable for LpushCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LpushCommand {
    fn clone(&self) -> LpushCommand {
        LpushCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LpushCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_list_db();
        //TODO: chequear el caso de que la clave sea de otro tipo

        let mut result = db.lpush(args).to_string();
        result.push('\n');

        Ok(result)
    }
}
