use crate::command::cmd_trait::Command;
use crate::command::command_parser::ParsedMessage;
use crate::command::constants::TYPE_LIST;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command LREM\n";
const CLIENT_ID: &str = "LremCommand";

const ZERO_RESULT: &str = "0";

const MIN_VALID_ARGS: u32 = 3;
const MAX_VALID_ARGS: u32 = 3;

pub struct LremCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl LremCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> LremCommand {
        LremCommand { id_job, logger }
    }
}

impl Loggable for LremCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LremCommand {
    fn clone(&self) -> LremCommand {
        LremCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LremCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let mut result;
        let db_resolver = app_info.get_db_resolver();
        /* The key for the DB */
        let key_str = args[0];
        match db_resolver.type_key(String::from(key_str)) {
            Ok(typee) => {
                if typee == *TYPE_LIST.to_string() {
                    let db = app_info.get_list_db();
                    result = db.lrem(args).to_string();
                } else {
                    return Err(RunError {
                        message: "ERR WRONGTYPE.".to_string(),
                        cause: "Operación incorrecta para el tipo de valor asociado a la clave."
                            .to_string(),
                    });
                }
            }
            Err(_e) => result = String::from(ZERO_RESULT),
        }

        result.push('\n');

        Ok(result)
    }
}
