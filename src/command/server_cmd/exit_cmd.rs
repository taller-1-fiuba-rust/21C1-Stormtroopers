use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::constants::MSG_OVER;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command EXIT\n";
const CLIENT_ID: &str = "ExitCommand";
const CONST_CMD: &str = "exit";

pub struct ExitCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl ExitCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for ExitCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for ExitCommand {
    fn clone(&self) -> ExitCommand {
        ExitCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for ExitCommand {
    fn run(
        &self,
        _args: Vec<&str>,
        app_info: &AppInfo,
        id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        app_info
            .get_connection_resolver()
            .disconnect_client(id_client);
        println!("Disconnecting client {:?}", id_client);

        Ok(MSG_OVER.to_string())
    }
}
