use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};
use crate::RESP_SIMPLE_STRING;

const INFO_FLUSHDB_COMMAND: &str = "Run command FLUSHDB\n";
const CLIENT_ID: &str = "FlushdbCommand";
const CONST_CMD: &str = "flushdb";

pub struct FlushdbCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl FlushdbCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for FlushdbCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for FlushdbCommand {
    fn clone(&self) -> FlushdbCommand {
        FlushdbCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for FlushdbCommand {
    fn run(
        &self,
        _args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self
            .logger
            .info(self, INFO_FLUSHDB_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_db_resolver();
        let res = match db.clean_all_data() {
            true => String::from(RESP_SIMPLE_STRING),
            false => panic!("Esto no deberia pasar!"),
        };

        Ok(res)
    }
}
