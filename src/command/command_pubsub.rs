use crate::app_info::AppInfo;
use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};
use crate::LINE_BREAK;

const INFO_RUN_COMMAND: &str = "Run command PUBSUB\n";

pub struct PubsubCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl PubsubCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> PubsubCommand {
        PubsubCommand { id_job, logger }
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

impl Command for PubsubCommand {
    fn run(&self, args: Vec<&str>, app_info: &AppInfo) -> Result<String, RunError> {
        let _log_info_res = self.logger.info(self, INFO_RUN_COMMAND);

        let arg = args[0];
        let mut response = "".to_string();

        let id_client = app_info.get_id_client();
        let mut pubsub = app_info.get_pubsub();
        match arg {
            "suscribe" => pubsub.suscribe(args[1].to_string(), id_client),
            "len_channel" => {
                let len: usize = pubsub.len_channel(args[1].to_string());
                response = format!("{:?}", len);
            }
            "suscribers_for_channel" => {
                let suscribers_vec = pubsub.get_suscribers(args[1].to_string());
                response = format!("{:?}", suscribers_vec);
            }
            "publish" => {
                pubsub.publish(args[1].to_string(), args[2].to_string());
            }
            "unsuscribe" => pubsub.unsuscribe(args[1].to_string(), id_client),
            "CHANNELS" => {
                let channels_vec = pubsub.available_channels();
                response = format!("{:?}", channels_vec);
                response.push(LINE_BREAK);
            }
            "NUMSUB" => {
                let vec = pubsub.numsub();
                response = format!("{:?}", vec);
                response.push(LINE_BREAK)
            }
            _ => {
                return Err(RunError {
                    message: "Error Command pubsub".to_string(),
                    cause: " ".to_string(),
                })
            }
        }

        Ok(response)
    }
}
