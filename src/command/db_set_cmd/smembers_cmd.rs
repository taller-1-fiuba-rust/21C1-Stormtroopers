use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command SMEMBERS\n";
const CLIENT_ID: &str = "SmembersCommmand";
const RESPONSE_EMPTY: &str = "(empty list or set)\n";
const CONST_CMD: &str = "smembers";

pub struct SmembersCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl SmembersCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for SmembersCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for SmembersCommand {
    fn clone(&self) -> SmembersCommand {
        SmembersCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for SmembersCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_set_db();

        let res_items = db.smembers(args);

        if res_items.is_empty() {
            return Ok(String::from(RESPONSE_EMPTY));
        }

        let mut to_return = String::from("");
        for (i, it) in res_items.iter().enumerate() {
            to_return.push_str(i.to_string().as_str());
            to_return.push_str(") ");
            let mut item = it.clone();
            item.push('\n');
            to_return.push_str(&item);
        }

        Ok(to_return)
    }
}
