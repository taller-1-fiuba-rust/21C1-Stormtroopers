use crate::command::cmd_trait::Command;
use crate::command::constants::TYPE_LIST;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command LPUSHX\n";
const CLIENT_ID: &str = "LpushxCommand";

const ZERO_RESULT: &str = "0";

pub struct LpushxCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl LpushxCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> LpushxCommand {
        LpushxCommand { id_job, logger }
    }
}

impl Loggable for LpushxCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LpushxCommand {
    fn clone(&self) -> LpushxCommand {
        LpushxCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LpushxCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        //TODO: falta chequear parametros?

        let mut result;
        let db_resolver = app_info.get_db_resolver();
        let key_str = args[0]; // The key for the DB
        match db_resolver.type_key(String::from(key_str)) {
            Ok(typee) => {
                if typee == *TYPE_LIST.to_string() {
                    let db = app_info.get_list_db();
                    result = db.lpush(args).to_string();
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
