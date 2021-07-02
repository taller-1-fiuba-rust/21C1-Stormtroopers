use crate::constants::END_FLAG;
use crate::handle_connection::handle_monitor::publish_monitor;
use crate::server::app_info::AppInfo;
use crate::Connection;

//TODO: ver porque si vienen mal los args explota
pub fn process_request(
    request: String,
    app_info: &AppInfo,
    _id_job: u32,
    id_client: usize,
) -> String {
    let command_builder = app_info.get_command_builder();

    let cmd = command_builder.get_command(&String::from(request.trim()));
    let mut command_splited: Vec<&str> = request.split(' ').collect();
    publish_monitor(app_info.clone(), command_splited.clone(), id_client);

    command_splited.remove(0);

    match cmd {
        Ok(cmd) => match cmd.run(command_splited, app_info, id_client) {
            Ok(res) => res,
            Err(res) => res.to_string(),
        },
        Err(cmd) => cmd.to_string(),
    }
}

pub fn run_exit_cmd(
    connect_client: Connection<String>,
    app_info: &AppInfo,
    id_job: u32,
    id_client: usize,
) {
    let response = process_request(END_FLAG.to_string(), app_info, id_job, id_client);
    connect_client.send(response);
}