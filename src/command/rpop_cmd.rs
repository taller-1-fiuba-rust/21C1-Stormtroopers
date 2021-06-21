use crate::command::cmd_trait::Command;
use crate::command::command_parser::ParsedMessage;
use crate::command::constants::TYPE_LIST;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command RPOP\n";
const CLIENT_ID: &str = "RpopCommand";

const NIL_RESULT: &str = "(nil)\n";

const MIN_VALID_ARGS: i32 = 1;
const MAX_VALID_ARGS: i32 = 2;

pub struct RpopCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl RpopCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> RpopCommand {
        RpopCommand { id_job, logger }
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
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let mut result = String::from("");
        let db_resolver = app_info.get_db_resolver();
        /* The key for the DB */
        let key_str = args[0].to_string();
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

        match db_resolver.type_key(key_str.clone()) {
            Ok(typee) => {
                if typee == *TYPE_LIST.to_string() {
                    let db = app_info.get_list_db();
                    let items = db.rpop(key_str, count);
                    if items.is_empty() {
                        result = String::from(NIL_RESULT);
                    } else {
                        for (i, it) in items.iter().enumerate() {
                            result.push_str((i + 1).to_string().as_str());
                            result.push_str(") ");
                            let mut item = it.clone();
                            item.push('\n');
                            result.push_str(&item);
                        }
                    }
                } else {
                    return Err(RunError {
                        message: "ERR WRONGTYPE.".to_string(),
                        cause: "Operación incorrecta para el tipo de valor asociado a la clave."
                            .to_string(),
                    });
                }
            }
            Err(_e) => result = String::from(NIL_RESULT),
        }

        Ok(result)
    }
}
