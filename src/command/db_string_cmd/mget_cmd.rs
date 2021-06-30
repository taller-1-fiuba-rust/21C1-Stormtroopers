use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command MGET\n";
const CLIENT_ID: &str = "MgetCommmand";

pub struct MgetCommmand {
    id_job: u32,
    logger: Logger<String>,
}

impl MgetCommmand {
    pub fn new(id_job: u32, logger: Logger<String>) -> MgetCommmand {
        MgetCommmand { id_job, logger }
    }
}

impl Loggable for MgetCommmand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for MgetCommmand {
    fn clone(&self) -> MgetCommmand {
        MgetCommmand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for MgetCommmand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let db_string = app_info.get_string_db();

        let res = db_string.mget(args);
        let mut to_return = String::from("");
        let mut i = 1;
        for res in res.iter() {
            to_return.push_str(i.to_string().as_str());
            to_return.push_str(") ");
            let mut item = res.clone();
            item.push('\n');
            to_return.push_str(&item);
            i += 1;
        }
        Ok(to_return)
    }
}
