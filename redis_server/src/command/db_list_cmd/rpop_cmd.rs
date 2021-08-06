//! Returns the last elements of the list (given by the user, defaults to 1), and removes them.
//!
//! Example:
//! ```text
//! > lrange key 0 -1
//! 0) value1
//! 1) value2
//! 2) value3
//! 3) value4
//! > rpop key
//! "value4"
//! > lrange key 0 -1
//! 0) value1
//! 1) value2
//! 2) value3
//! > rpop key 2
//! 0) value2
//! 1) value3
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, NIL_RESPONSE, TYPE_LIST};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_COMMAND: &str = "Run command RPOP\n";

/// Name of the command.
const CLIENT_ID: &str = "RpopCommand";

/// Code of the command.
const RPOP_CMD: &str = "rpop";

/// Min amount of arguments besides of the command.
const MIN_VALID_ARGS: i32 = 1;

/// Max amount of arguments besides of the command.
const MAX_VALID_ARGS: i32 = 2;

/// Main struct of the command.
pub struct RpopCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl RpopCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(RPOP_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for RpopCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for RpopCommand {
    fn clone(&self) -> RpopCommand {
        RpopCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for RpopCommand {
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
        app_info.get_db_resolver().valid_key_type(key, TYPE_LIST)?;

        //TODO: sacar esta logica a fn en comun
        let mut count = 1_u32;
        if args.len() == 2 {
            count = match args[1].parse::<u32>() {
                Ok(count) => count,
                Err(_e) => {
                    return Err(RunError {
                        message: "ERR.".to_string(),
                        cause: "El valor esta fuera de rango.Debe ser positivo.".to_string(),
                    })
                }
            }
        };

        let db = app_info.get_list_db_sharding(key);
        let items = db.rpop(key.to_string(), count);

        //TODO: sacar esta logica a fn en comun
        let mut result = "".to_string();
        if items.is_empty() {
            result = NIL_RESPONSE.to_string();
        } else {
            for (i, it) in items.iter().enumerate() {
                result.push_str((i + 1).to_string().as_str());
                result.push_str(") ");
                let mut item = it.clone();
                item.push(LINE_BREAK);
                result.push_str(&item);
            }
        }

        Ok(result)
    }
}
