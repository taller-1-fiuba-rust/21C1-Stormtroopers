use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command LSET\n";
const CLIENT_ID: &str = "LsetCommand";
const LSET_CMD: &str = "lset";

pub struct LsetCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl LsetCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(LSET_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for LsetCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LsetCommand {
    fn clone(&self) -> LsetCommand {
        LsetCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LsetCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_list_db();

        let mut result = db.lset(
            args[0].to_string(),
            args[1].to_string(),
            args[2].to_string(),
        )?;
        result.push('\n');

        Ok(result)
    }
}
