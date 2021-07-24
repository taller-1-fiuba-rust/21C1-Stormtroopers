//! Given a key, resets last access time and, triggers cleanup if ttl is expired for that key.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::command::command_parser::ParsedMessage;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_COMMAND: &str = "Run command TOUCH\n";
const CLIENT_ID: &str = "TouchCommand";
const CONST_CMD: &str = "touch";

const MIN_VALID_ARGS: i32 = 1;
const MAX_VALID_ARGS: i32 = -1;

pub struct TouchCommand {
    /// Id of the thread running.
    id_job: u32,
    /// Logger entity.
    logger: Logger<String>,
}

impl TouchCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for TouchCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for TouchCommand {
    fn clone(&self) -> TouchCommand {
        TouchCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for TouchCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self.logger.info(self, INFO_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let db = app_info.get_db_resolver();

        let mut vec_args = Vec::<String>::new();
        for arg in args {
            vec_args.push(arg.to_string());
        }
        let response_vec = db.touch(vec_args.clone());

        for (i, touch_key) in response_vec.iter().enumerate() {
            if *touch_key != 0 {
                self.logger
                    .info(
                        self,
                        &generate_info(vec_args[i].clone(), *touch_key),
                        app_info.get_verbose(),
                    )
                    .unwrap();
            } else {
                self.logger
                    .info(
                        self,
                        &without_touch_key(vec_args[i].clone()),
                        app_info.get_verbose(),
                    )
                    .unwrap();
            }
        }

        //chequeo el ttl
        let mut app_info = app_info.clone();
        for key in vec_args {
            app_info.check_ttl_key(key)?;
        }

        Ok(generate_response(response_vec))
    }
}

pub fn generate_info(key: String, touch_key: u64) -> String {
    format!(
        "Run command TOUCH for key {} - Last access: {} secs\n",
        key, touch_key
    )
}

pub fn without_touch_key(key: String) -> String {
    format!("Run command TOUCH for key {} - Key doesn't exist", key)
}

pub fn generate_response(response_vec: Vec<u64>) -> String {
    let mut count = 0;
    for touch_key in response_vec {
        if touch_key != 0 {
            count += 1;
        }
    }
    format!("{}\n", count)
}
