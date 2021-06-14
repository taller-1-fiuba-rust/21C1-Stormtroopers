use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};

const INFO_EXPIRE_COMMAND: &str = "Run command TTL\n";
const CLIENT_ID: &str = "ExpireCommmand";
const WRONG_NUMBER_ARGUMENTS: &str = "Wrong number of arguments.\n";
const NIL: &str = "(nil)";
const NOT_FOUND: &str = "Key not found.\n";
const WHITESPACE: &str = " ";
const OK: &str = "OK.\n";

pub struct PersistCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl PersistCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> PersistCommand {
        PersistCommand { id_job, logger }
    }
}

impl Loggable for PersistCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for PersistCommand {
    fn clone(&self) -> PersistCommand {
        PersistCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for PersistCommand {
    fn run(&self, args: Vec<&str>, app_info: &AppInfo) -> Result<String, RunError> {
        let _log_info_res = self.logger.info(self, INFO_EXPIRE_COMMAND);
        // First, check number of args.
        // Second, check if the key is present. If true, set the ttl, if not, do nothing.
        if args.len() != 1 {
            return Err(RunError{message: args.join(WHITESPACE), cause: String::from(WRONG_NUMBER_ARGUMENTS)});
        }
        
        let structure = app_info.get_structure();
        let string = structure.get_string(String::from(args[0]));
        match string.as_str() {
            NIL => Ok(String::from(NOT_FOUND)),
            _ => {
                let ttl_scheduler = app_info.get_ttl_scheduler();
                match ttl_scheduler.delete_ttl_helper(String::from(args[0])) {
                    Ok(val) => {
                        ttl_scheduler.delete_ttl(val).unwrap_or(String::from(""));
                        Ok(String::from(OK))
                    },
                    Err(_) => Ok(String::from(OK)),
                }
            }
        }
    }
}