use crate::constants::LINE_BREAK;
use crate::AppInfo;
use std::time::SystemTime;

const SECS_IN_DAY: u64 = 86400;

/* Application statistics */
pub struct SystemInfo {
    start_time: SystemTime,
    actives_threads: usize,
    max_clients: usize,
}
///Statistics application. Retrieves and formats statistical information from the server.
///
/// It is used for the "info" command of the server group.
impl SystemInfo {
    pub fn new(max_clients: usize) -> Self {
        Self {
            start_time: SystemTime::now(),
            actives_threads: 0,
            max_clients,
        }
    }

    pub fn info(&self, app_info: AppInfo, process_id: usize) -> String {
        let mut response = LINE_BREAK.to_string();

        response.push_str(&self.connected_clients(app_info.clone()));
        response.push_str(&self.tcp_port(app_info.clone()));
        response.push_str(&self.process_id(process_id));
        response.push_str(&self.max_clients());
        response.push_str(&self.uptime());
        response.push_str(&self.actives_threads(app_info.clone()));
        response.push_str(&self.config_file(app_info));

        response.push(LINE_BREAK);
        response
    }

    fn connected_clients(&self, app_info: AppInfo) -> String {
        format!(
            "-> connected_clients: {}\n",
            app_info.get_connected_clients()
        )
    }

    fn tcp_port(&self, app_info: AppInfo) -> String {
        format!("-> tcp_port: {}\n", app_info.get_server_port())
    }

    fn process_id(&self, process_id: usize) -> String {
        format!("-> process_id: {}\n", process_id)
    }

    fn max_clients(&self) -> String {
        format!("-> max_clients: {}\n", self.max_clients)
    }

    fn uptime(&self) -> String {
        let uptime_secs = SystemTime::now()
            .duration_since(self.start_time)
            .expect("duration error")
            .as_secs();
        let uptime_days = uptime_secs / SECS_IN_DAY;
        format!(
            "-> uptime: {}\n-> uptime_days: {}\n",
            uptime_secs, uptime_days
        )
    }

    fn actives_threads(&self, app_info: AppInfo) -> String {
        format!(
            "-> actives_threads: {}\n",
            app_info.get_connected_clients() * 2
        )
    }

    fn config_file(&self, app_info: AppInfo) -> String {
        let mut response = "-> Config Server:\n".to_string();
        response.push_str(&app_info.get_config_server().get());
        response
    }

    pub fn activate_thread(&mut self) {
        self.actives_threads += 1;
    }

    pub fn deactivate_thread(&mut self) {
        if self.actives_threads > 0 {
            self.actives_threads -= 1;
        }
    }

    pub fn get_actives_threads(&self) -> usize {
        self.actives_threads
    }
}

impl Clone for SystemInfo {
    fn clone(&self) -> Self {
        Self {
            start_time: self.start_time,
            actives_threads: self.actives_threads,
            max_clients: self.max_clients,
        }
    }
}
