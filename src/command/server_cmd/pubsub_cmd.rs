//! A command group that ables the client to create and subscribe to channels of information.
use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};
const INFO_RUN_COMMAND: &str = "Run command PUBSUB\n";

const CONST_CMD: &str = "pubsub";

pub struct PubsubCommand {
    id_job: u32,
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
        id_client: usize,
    ) -> Result<String, RunError> {
        let _log_info_res = self
            .logger
            .info(self, INFO_RUN_COMMAND, app_info.get_verbose());

        let arg = args[0];
        let mut response = "".to_string();

        let mut pubsub = app_info.get_pubsub();

        match arg {
            "suscribe" => {
                pubsub.suscribe(args[1].to_string(), id_client);
                response = "OK\n".to_string();
            }
            "len_channel" => {
                let len: usize = pubsub.len_channel(args[1].to_string());
                response = format!("{:?}\n", len);
            }
            "suscribers_for_channel" => {
                let suscribers_vec = pubsub.get_suscribers(args[1].to_string());
                response = format!("{:?}\n", suscribers_vec);
            }
            "publish" => {
                let val = pubsub.publish(args[1].to_string(), args[2].to_string(), false);
                response = "OK\n".to_string();
                if val.is_none() {
                    return Err(RunError {
                        message: "Error Command pubsub".to_string(),
                        cause: "Channel does not exist".to_string(),
                    });
                }
            }
            "unsuscribe" => pubsub.unsuscribe(args[1].to_string(), id_client),
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
