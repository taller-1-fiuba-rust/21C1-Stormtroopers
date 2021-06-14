use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};
use crate::LINE_BREAK;

const INFO_COMMAND: &str = "Run command TYPE\n";
const CLIENT_ID: &str = "TypeCommand";

pub struct TypeCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl TypeCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> TypeCommand {
        TypeCommand { id_job, logger }
    }
}

impl Loggable for TypeCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for TypeCommand {
    fn clone(&self) -> TypeCommand {
        TypeCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for TypeCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_db_resolver();

        let mut response = db.type_key(args[0].to_string())?;
        response.push(LINE_BREAK);

        Ok(response)
    }
}
