use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command COPY\n";
const CLIENT_ID: &str = "CopyCommmand";
const CONST_CMD: &str = "copy";

pub struct CopyCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl CopyCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for CopyCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for CopyCommand {
    fn clone(&self) -> CopyCommand {
        CopyCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for CopyCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let mut db = app_info.get_string_db();

        let resp = db.copy(String::from(args[0]), String::from(args[1]));
        let mut resp_str = resp.to_string();
        resp_str.push('\n');
        Ok(resp_str)
    }
}
