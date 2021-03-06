//! Finds keys matching a given pattern.
//!
//! Example:
//! ```text
//! > mset key1 v1 key2 v2 key3 v3 other v4
//! OK
//! > keys *
//! 0) key1
//! 1) key2
//! 2) key3
//! 4) other
//! > keys key
//! 0) key1
//! 1) key2
//! 2) key3
//! > keys e
//! 0) key1
//! 1) key2
//! 2) key3
//! 4) other
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::LINE_BREAK;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command KEYS\n";

/// Name of the command.
const CLIENT_ID: &str = "KeysCommand";

/// Code of the command.
const CONST_CMD: &str = "keys";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 1;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = -1;

/// Main struct of the command.
pub struct KeysCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl KeysCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for KeysCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for KeysCommand {
    fn clone(&self) -> KeysCommand {
        KeysCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for KeysCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let db = app_info.get_db_resolver();
        let keys = db.keys(&String::from(args[0]));
        let mut response = "".to_string();

        match keys {
            Ok(vec) => {
                for (i, key) in vec.iter().enumerate() {
                    response.push_str(&format!("{}) ", i + 1));
                    response.push_str(&key);
                    response.push(LINE_BREAK);
                }
            }
            Err(e) => response = e.to_string(),
        }

        Ok(response)
    }
}
