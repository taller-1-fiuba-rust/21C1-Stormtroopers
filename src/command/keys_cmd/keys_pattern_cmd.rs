//! Finds keys matching a given pattern.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::constants::LINE_BREAK;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command KEYS\n";
const CLIENT_ID: &str = "KeysCommand";
const CONST_CMD: &str = "keys";

pub struct KeysCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl KeysCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for KeysCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for KeysCommand {
    fn clone(&self) -> KeysCommand {
        KeysCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for KeysCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_db_resolver();
        let keys = db.keys(&String::from(args[0]));
        let mut response = "".to_string();

        for key in keys {
            response.push_str(&key);
            response.push(LINE_BREAK);
        }

        /*probar
        set hola@ 21
        keys ^(?P<login>[^@\s]+)@
        */
        Ok(response)
    }
}
