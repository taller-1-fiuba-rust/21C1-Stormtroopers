use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::utils::timestamp_now;
use crate::logger::{Loggable, Logger};

const INFO_EXPIREAT_COMMAND: &str = "Run command EXPIRE_AT\n";
const CLIENT_ID: &str = "ExpireAtCommmand";
const WRONG_NUMBER_ARGUMENTS: &str = "Wrong number of arguments.\n";
const WRONG_TTL_TYPE: &str = "Can't parse to expire time.\n";
const NIL: &str = "(nil)";
const NOT_FOUND: &str = "Key not found.\n";
const WHITESPACE: &str = " ";

pub struct ExpireAtCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl ExpireAtCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> ExpireAtCommand {
        ExpireAtCommand { id_job, logger }
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
    fn run(&self, args: Vec<&str>, app_info: &AppInfo) -> Result<String, RunError> {
        let _log_info_res = self.logger.info(self, INFO_EXPIREAT_COMMAND);
        // First, check number of args.
        // Second, check if the key is present. If true, set the ttl, if not, do nothing.
        if args.len() != 2 {
            return Err(RunError{message: args.join(WHITESPACE), cause: String::from(WRONG_NUMBER_ARGUMENTS)});
        }
        
        let structure = app_info.get_structure();
        let string = structure.get_string(String::from(args[0]));
        match string.as_str() {
            NIL => Ok(String::from(NOT_FOUND)),
            _ => {
                let ttl_scheduler = app_info.get_ttl_scheduler();
                let ttl = args[1].parse::<u64>(); // This ttl is in unix time.
                match ttl {
                    Ok(ttl) => { // Maybe check if ttl is less than actual timestamp?
                        if ttl <= timestamp_now() {
                            return Err(RunError{message: args.join(WHITESPACE), cause: String::from(WRONG_TTL_TYPE)});
                        }
                        let mut return_value = ttl_scheduler.set_ttl(ttl, String::from(args[0])).unwrap();
                        return_value.push('\n');
                        Ok(return_value)
                    },
                    Err(_) => Err(RunError{message: args.join(WHITESPACE), cause: String::from(WRONG_TTL_TYPE)}),
                }
            }
        }
    }
}
