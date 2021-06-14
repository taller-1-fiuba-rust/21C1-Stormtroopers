use crate::command::cmd_trait::Command;
use crate::command::constants::LRANGE_COMMAND_STR;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command LRANGE\n";
const CLIENT_ID: &str = "LrangeCommmand";
const RESPONSE_EMPTY: &str = "(empty list or set)\n";

pub struct LrangeCommmand {
    id_job: u32,
    logger: Logger<String>,
}

impl LrangeCommmand {
    pub fn new(id_job: u32, logger: Logger<String>) -> LrangeCommmand {
        LrangeCommmand { id_job, logger }
    }
}

impl Loggable for LrangeCommmand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for LrangeCommmand {
    fn clone(&self) -> LrangeCommmand {
        LrangeCommmand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for LrangeCommmand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        //TODO: ver si conviene poner esta logica en otro lado
        if args.len() != 3 {
            let msg_err = format!(
                "(error) ERR wrong number of arguments for '{}' command",
                LRANGE_COMMAND_STR
            );
            return Err(RunError {
                message: msg_err,
                cause: String::from(""),
            });
        }

        let db = app_info.get_list_db();

        let items = db.lrange(args);

        if items.is_empty() {
            return Ok(String::from(RESPONSE_EMPTY));
        }

        //TODO: codigo duplicado
        let mut to_return = String::from("");
        for (i, it) in items.iter().enumerate() {
            to_return.push_str(i.to_string().as_str());
            to_return.push_str(") ");
            let mut item = it.clone();
            item.push('\n');
            to_return.push_str(&item);
        }

        Ok(to_return)
    }
}
