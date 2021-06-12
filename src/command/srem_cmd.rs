use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command SREM\n";
const CLIENT_ID: &str = "SremCommmand";

pub struct SremCommmand {
    id_job: u32,
    logger: Logger<String>,
}

impl SremCommmand {
    pub fn new(id_job: u32, logger: Logger<String>) -> SremCommmand {
        SremCommmand { id_job, logger }
    }
}

impl Loggable for SremCommmand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for SremCommmand {
    fn clone(&self) -> SremCommmand {
        SremCommmand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for SremCommmand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND);
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_set_db();

        //TODO: verificar si la clave encontrada no es de otro tipo. Si lo es levantar Error

        let mut items_removed = db.srem(args).to_string();

        items_removed.push('\n');

        Ok(items_removed)
    }
}
