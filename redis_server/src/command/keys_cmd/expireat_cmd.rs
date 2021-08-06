//! Sets an expiration time for a key, given with a valid UNIX timestamp in seconds.
//!
//! Example:
//! ```text
//! > set key value
//! OK
//! > expireat key 1627169941
//! OK
//! > ttl key
//! 60
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::LINE_BREAK;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};
use crate::server::utils::timestamp_now;

/// Information string of the command.
const INFO_EXPIREAT_COMMAND: &str = "Run command EXPIRE_AT\n";

/// Name of the command.
const CLIENT_ID: &str = "ExpireAtCommand";

/// Failure string for the command.
const WRONG_TTL_TYPE: &str = "Can't parse to expire time.\n";

/// A whitespace string.
const WHITESPACE: &str = " ";

/// Code of the command.
const CONST_CMD: &str = "expireat";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 2;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 2;

/// Main struct of the command.
pub struct ExpireAtCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl ExpireAtCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for ExpireAtCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for ExpireAtCommand {
    fn clone(&self) -> ExpireAtCommand {
        ExpireAtCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for ExpireAtCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let _log_info_res = self
            .logger
            .info(self, INFO_EXPIREAT_COMMAND, app_info.get_verbose());

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let key_str = args[0]; // The key for the DB
        let ttl_str = args[1]; // The ttl
        let db = app_info.get_db_resolver();

        match db.type_key(String::from(key_str)) {
            Ok(_db_type) => {
                let ttl_scheduler = app_info.get_ttl_scheduler();
                match ttl_str.parse::<u64>() {
                    Ok(ttl) => {
                        if ttl <= timestamp_now() {
                            return Err(RunError {
                                message: args.join(WHITESPACE),
                                cause: String::from(WRONG_TTL_TYPE),
                            });
                        }
                        let mut return_value =
                            ttl_scheduler.set_ttl(ttl, String::from(key_str)).unwrap();
                        return_value.push(LINE_BREAK);
                        Ok(return_value)
                    }
                    Err(_) => Err(RunError {
                        message: args.join(WHITESPACE),
                        cause: String::from(WRONG_TTL_TYPE),
                    }),
                }
            }
            Err(e) => Err(e),
        }
    }
}
