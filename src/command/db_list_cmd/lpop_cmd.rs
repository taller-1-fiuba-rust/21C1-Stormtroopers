use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::constants::{LINE_BREAK, TYPE_LIST};
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command LPOP\n";
const CLIENT_ID: &str = "LpopCommand";
const RESPONSE_EMPTY: &str = "(nil)\n";
const LPOP_CMD: &str = "lpop";

const MIN_VALID_ARGS: i32 = 1;
const MAX_VALID_ARGS: i32 = 2;

pub struct LpopCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl LpopCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(LPOP_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for LpopCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LpopCommand {
    fn clone(&self) -> LpopCommand {
        LpopCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}
//TODO: falta impl con el modificador count para que retorne un rango de elementos
impl Command for LpopCommand {
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
        let db = app_info.get_list_db_sharding(key);

        app_info.get_db_resolver().valid_key_type(key, TYPE_LIST)?;

        let result = db.lpop(args);

        if result.is_empty() {
            return Ok(String::from(RESPONSE_EMPTY));
        }

        let mut to_return = result[0].clone();
        to_return.push(LINE_BREAK);
        Ok(to_return)
    }
}
