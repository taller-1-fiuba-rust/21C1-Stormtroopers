use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command MSET\n";
const CLIENT_ID: &str = "MsetCommand";
const RESPONSE_COMMAND: &str = "OK\n";
const CONST_CMD: &str = "mset";

const MIN_VALID_ARGS: i32 = 2;
const MAX_VALID_ARGS: i32 = -1;

pub struct MsetCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl MsetCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for MsetCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for MsetCommand {
    fn clone(&self) -> MsetCommand {
        MsetCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

//TODO: ver Impl! Parece dificil hacerlo atómico
impl Command for MsetCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;
        /* Validate num of params */
        if args.len() % 2 != 0 {
            let msg_err = "Number of arguments must be pair".to_string();
            return Err(RunError {
                message: "ERROR NUMBER OR ARGUMENTS.".to_string(),
                cause: msg_err,
            });
        }

        let mut i = 0;
        while i < args.len() {
            let key = args[i];
            let val = args[i + 1];
            let db_string = app_info.get_string_db_sharding(key);
            db_string.set_string(key.to_string(), val.to_string());
            i += 2;
        }
        Ok(String::from(RESPONSE_COMMAND))
    }
}
