//! A command group that ables the client to create and subscribe to channels of information.
//!
//! Example pubsub suscribe:
//! ```text
//! > pubsub suscribe channel1
//! OK
//! ```
//! Example pubsub publish:
//! ```text
//! > pubsub publish channel1 "Hello world!"
//! OK
//! ```
//! Example pubsub channels:
//! ```text
//! > pubsub channels
//! ["channel1"]
//! ```
//! Example pubsub unsuscribe:
//! ```text
//! > pubsub unsuscribe channel1
//!
//! ```
//! Example pubsub numsub:
//! ```text
//! > pubsub numsub
//! [("channel1", 1)]
//! ```
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

/// Information string to log.
const INFO_RUN_COMMAND: &str = "Run command PUBSUB\n";

/// Code of the command.
const CONST_CMD: &str = "pubsub";

/// Main structure of the command.
pub struct PubsubCommand {
    /// Id of the thread running.
    id_job: u32,
    ///
    logger: Logger<String>,
}

impl PubsubCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for PubsubCommand {
    fn get_id_client(&self) -> &str {
        "Pubsub"
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for PubsubCommand {
    fn clone(&self) -> PubsubCommand {
        PubsubCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for PubsubCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let _log_info_res = self
            .logger
            .info(self, INFO_RUN_COMMAND, app_info.get_verbose());

        let arg = args[0];
        let response;

        let pubsub = app_info.get_pubsub();

        match arg {
            "CHANNELS" | "channels" => {
                if args.len() == 1 {
                    let channels_vec = pubsub.available_channels();
                    response = format!("{:?}\n", channels_vec);
                } else {
                    let channels_vec = pubsub.available_channels_pattern(args[1]);
                    response = format!("{:?}\n", channels_vec);
                }
            }
            "NUMSUB" | "numsub" => {
                let vec = pubsub.numsub();
                response = format!("{:?}\n", vec);
            }
            _ => {
                return Err(RunError {
                    message: "Error Command pubsub".to_string(),
                    cause: format!("{:?} is not a pubsub command", arg),
                });
            }
        }

        Ok(response)
    }
}
