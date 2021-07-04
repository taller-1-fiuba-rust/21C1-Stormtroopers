use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command MGET\n";
const CLIENT_ID: &str = "MgetCommand";
const CONST_CMD: &str = "mget";

pub struct MgetCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl MgetCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for MgetCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for MgetCommand {
    fn clone(&self) -> MgetCommand {
        MgetCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for MgetCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let db_string = app_info.get_string_db();

        let res = db_string.mget(args);
        let mut to_return = String::from("");
        let mut i = 1;
        for res in res.iter() {
            to_return.push_str(i.to_string().as_str());
            to_return.push_str(") ");
            let mut item = res.clone();
            item.push('\n');
            to_return.push_str(&item);
            i += 1;
        }
        Ok(to_return)
    }
}
