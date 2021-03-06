//! Returns the expire time of a key.
//!
//! Example:
//! ```text
//! > expire key 60
//! OK
//! > ttl key
//! 60
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};
use crate::server::utils::timestamp_now;

/// Information string to log.
const INFO_EXPIRE_COMMAND: &str = "Run command TTL\n";

/// Name of the command.
const CLIENT_ID: &str = "ExpireCommand";

/// Response string for the command.
const TTL_ZERO_OR_ABSENT: &str = "-2\n";

/// Newline character.
const NEW_LINE: &str = "\n";

/// Code of the command.
const CONST_CMD: &str = "ttl";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 1;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 1;

/// Main struct for the command.
pub struct TtlCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl TtlCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for TtlCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for TtlCommand {
    fn clone(&self) -> TtlCommand {
        TtlCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for TtlCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let _log_info_res = self
            .logger
            .info(self, INFO_EXPIRE_COMMAND, app_info.get_verbose());

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let key_str = args[0]; // The key for the DB
        let db = app_info.get_db_resolver();

        match db.type_key(String::from(key_str)) {
            Ok(_db_type) => {
                let ttl_scheduler = app_info.get_ttl_scheduler();
                let now = timestamp_now();
                match ttl_scheduler.get_ttl_key(String::from(key_str)) {
                    Ok(ttl) => match ttl.parse::<u64>().unwrap().overflowing_sub(now) {
                        (res, false) => {
                            let mut ret_value = res.to_string();
                            ret_value.push_str(NEW_LINE);
                            Ok(ret_value)
                        }
                        (_, true) => Ok(String::from(TTL_ZERO_OR_ABSENT)),
                    },
                    Err(_) => Ok(String::from(TTL_ZERO_OR_ABSENT)),
                }
            }
            Err(e) => Err(e),
        }
    }
}
