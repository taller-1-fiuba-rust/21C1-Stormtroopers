use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};
use crate::server::utils::timestamp_now;

const INFO_EXPIREAT_COMMAND: &str = "Run command EXPIRE_AT\n";
const CLIENT_ID: &str = "ExpireAtCommmand";
const WRONG_NUMBER_ARGUMENTS: &str = "Wrong number of arguments.\n";
const WRONG_TTL_TYPE: &str = "Can't parse to expire time.\n";
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
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let _log_info_res = self.logger.info(self, INFO_EXPIREAT_COMMAND);

        if args.len() != 2 {
            return Err(RunError {
                message: args.join(WHITESPACE),
                cause: String::from(WRONG_NUMBER_ARGUMENTS),
            });
        }

        let key_str = args[0]; // The key for the DB
        let ttl_str = args[1]; // The ttl
        let db = app_info.get_db_resolver();

        match db.type_key(String::from(key_str)) {
            Ok(_db_type) => {
                let ttl_scheduler = app_info.get_ttl_scheduler();
                match ttl_str.parse::<u64>() {
                    Ok(ttl) => {
                        if ttl <= timestamp_now() {
                            return Err(RunError {
                                message: args.join(WHITESPACE),
                                cause: String::from(WRONG_TTL_TYPE),
                            });
                        }
                        let mut return_value =
                            ttl_scheduler.set_ttl(ttl, String::from(key_str)).unwrap();
                        return_value.push('\n');
                        Ok(return_value)
                    }
                    Err(_) => Err(RunError {
                        message: args.join(WHITESPACE),
                        cause: String::from(WRONG_TTL_TYPE),
                    }),
                }
            }
            Err(e) => Err(e),
        }
    }
}
