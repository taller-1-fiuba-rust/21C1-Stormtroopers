use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};
use crate::utils::timestamp_now;

const INFO_EXPIRE_COMMAND: &str = "Run command TTL\n";
const CLIENT_ID: &str = "ExpireCommmand";
const WRONG_NUMBER_ARGUMENTS: &str = "Wrong number of arguments.\n";
const WRONG_TTL_TYPE: &str = "Can't parse to expire time.\n";
const NIL: &str = "(nil)";
const NOT_FOUND: &str = "Key not found.\n";
const WHITESPACE: &str = " ";

pub struct TtlCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl TtlCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> TtlCommand {
        TtlCommand { id_job, logger }
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
                let now = timestamp_now();
                match ttl_scheduler.get_ttl(String::from(args[0])) {
                    Ok(val) => {
                        let delta_ttl = val.parse::<u64>().unwrap() - now;
                        let mut delta_ttl_str = delta_ttl.to_string();
                        delta_ttl_str.push('\n');
                        Ok(delta_ttl_str)
                    },
                    Err(_) => Err(RunError{message: args.join(WHITESPACE), cause: String::from(WRONG_TTL_TYPE)}),
                }
            }
        }
    }
}
