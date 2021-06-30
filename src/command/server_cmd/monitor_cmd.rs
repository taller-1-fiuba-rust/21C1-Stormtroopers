use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

use crate::LINE_BREAK;

const INFO_COMMAND: &str = "Run command MONITOR\n";
const CLIENT_ID: &str = "MonitorCommand";
const CONST_CMD: &str = "monitor";

pub struct MonitorCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl MonitorCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for MonitorCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for MonitorCommand {
    fn clone(&self) -> MonitorCommand {
        MonitorCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for MonitorCommand {
    fn run(
        &self,
        _args: Vec<&str>,
        app_info: &AppInfo,
        id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let mut private_pubsub = app_info.get_private_pubsub();
        private_pubsub.suscribe("MONITOR".to_string(), id_client);

        app_info
            .get_connection_resolver()
            .activate_monitor(id_client);
        let mut response = "OK".to_string();
        response.push(LINE_BREAK);

        Ok(response)
    }
}
