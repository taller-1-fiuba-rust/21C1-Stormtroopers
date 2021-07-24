//! Gets the values of several keys passed as arguments.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, MARK_BULLET};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command MGET\n";

/// Name of the command.
const CLIENT_ID: &str = "MgetCommand";
const CONST_CMD: &str = "mget";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 1;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = -1;

pub struct MgetCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
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
//TODO: review impl!
impl Command for MgetCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let mut res = vec![];
        for key in args {
            let db_string = app_info.get_string_db_sharding(key);
            let mut v = db_string.mget(vec![key]);
            res.append(&mut v);
        }
        let mut to_return = "".to_string();
        let mut i = 1;
        for res in res.iter() {
            to_return.push_str(i.to_string().as_str());
            to_return.push_str(MARK_BULLET);
            let mut item = res.clone();
            item.push(LINE_BREAK);
            to_return.push_str(&item);
            i += 1;
        }
        Ok(to_return)
    }
}
