use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};
use crate::utils::timestamp_now;

const INFO_EXPIRE_COMMAND: &str = "Run command EXPIRE\n";
const CLIENT_ID: &str = "ExpireCommmand";
const WRONG_NUMBER_ARGUMENTS: &str = "Wrong number of arguments.";
const WRONG_TTL_TYPE: &str = "Can't parse to expire time.";
const NIL: &str = "(nil)";
const NOT_FOUND: &str = "Key not found.";
const WHITESPACE: &str = " ";

pub struct ExpireCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl ExpireCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> ExpireCommand {
        ExpireCommand { id_job, logger }
    }
}

impl Loggable for ExpireCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for ExpireCommand {
    fn clone(&self) -> ExpireCommand {
        ExpireCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for ExpireCommand {
    fn run(&self, args: Vec<&str>, app_info: &AppInfo) -> Result<String, RunError> {
        let _log_info_res = self.logger.info(self, INFO_EXPIRE_COMMAND);
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
                let ttl = args[1].parse::<u64>(); // This ttl is in seconds.
                match ttl {
                    Ok(ttl) => {
                        let time_now = timestamp_now();
                        if ttl <= 0 {
                            return Err(RunError{message: args.join(WHITESPACE), cause: String::from(WRONG_TTL_TYPE)});
                        }
                        return ttl_scheduler.set_ttl(ttl + time_now, String::from(args[0]))
                    },
                    Err(_) => Err(RunError{message: args.join(WHITESPACE), cause: String::from(WRONG_TTL_TYPE)}),
                }}
        }
    }
}
