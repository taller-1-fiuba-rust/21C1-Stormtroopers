use crate::command::cmd_trait::Command;
use crate::server::app_info::AppInfo;
//use crate::db_resolver::get_string;
use crate::errors::run_error::RunError;
use crate::server::logger::{Loggable, Logger};

const INFO_RUN_COMMAND: &str = "Run command CONFIG\n";
const CLIENT_ID: &str = "ConfigCommand";

pub struct ConfigCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl ConfigCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> ConfigCommand {
        ConfigCommand { id_job, logger }
    }
}

impl Loggable for ConfigCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for ConfigCommand {
    fn clone(&self) -> ConfigCommand {
        ConfigCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for ConfigCommand {
    fn run(
        &self,
        mut args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_RUN_COMMAND);
        if let Ok(_r) = log_info_res {}

        let config_server = app_info.get_config_server();
        let _cmd = args[0];
        args.remove(0);

        let response: String;
        /*if cmd == "get" {
            response = config_server.get();
        } else if cmd == "set" {
            response = config_server.set(args);
        } else {
            //devolver error
        }*/
        response = config_server.get();

        Ok(response)
    }
}
