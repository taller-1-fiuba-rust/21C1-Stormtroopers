//! Prints the arguments in a human readable format.
use crate::server::app_info::AppInfo;
use crate::server::utils::format_timestamp_now;

pub fn publish_monitor(app_info: AppInfo, args: Vec<&str>, id_client: usize) {
    let priv_pubsub = app_info.get_private_pubsub();
    let port = app_info.get_server_port();

    let mut msg = format!(
        "+{:?} [id: {:?} -- port: {:?}] ",
        format_timestamp_now(),
        id_client,
        port
    );

    for arg in args {
        let msg_aux = format!("{:?} ", arg);
        msg.push_str(&msg_aux);
    }

    priv_pubsub.publish("MONITOR".to_string(), msg, true);
}
