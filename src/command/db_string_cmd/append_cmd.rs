//! Agrega el valor al final del String dado por la clave. Si no eiste, se crea una nueva.
//!
//! Example:
//! ```text
//! > set key value
//! OK
//! > append key 1
//! 6
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, TYPE_STRING};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command APPEND\n";

/// Name of the command.
const CLIENT_ID: &str = "AppendCommand";

/// Code of the command.
const CONST_CMD: &str = "append";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 2;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 2;

/// Main struct of the command.
pub struct AppendCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl AppendCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for AppendCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for AppendCommand {
    fn clone(&self) -> AppendCommand {
        AppendCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for AppendCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let key = args[0];
        app_info
            .get_db_resolver()
            .valid_key_type(&key.to_string(), TYPE_STRING)?;
        let db = app_info.get_string_db_sharding(key);

        let mut len_val_str = db.append(key.to_string(), args[1].to_string()).to_string();
        len_val_str.push(LINE_BREAK);

        Ok(len_val_str)
    }
}
