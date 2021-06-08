use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};

const INFO_EXPIREAT_COMMAND: &str = "Run command EXPIRE_AT\n";
const CLIENT_ID: &str = "ExpireAtCommmand";

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

        if args.len() < 2 {
            return Err(RunError{message: String::from(args[0]), cause: String::from("Couldn't parse timestamp from string.")});
        }

        let timestamp = args[0].parse::<u64>(); // timestamp en u64
        match timestamp {
            Ok(t) => {
                app_info.get_ttl_scheduler().add_ttl(t, args[1]);
                Ok(String::from("OK"))
            },
            Err(_) => Err(RunError{message: String::from(args[0]), cause: String::from("Couldn't parse timestamp from string.")})
        }
        
    }
}
