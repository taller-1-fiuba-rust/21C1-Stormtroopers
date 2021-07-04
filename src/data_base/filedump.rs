use crate::constants::{DBDUMP_INTERVAL_SECS, DBDUMP_PATH, ERROR_DBFILE_CREATE};
use crate::server::app_info::AppInfo;
use crate::server::utils::timestamp_now;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::SeekFrom;

fn generate_path(fname: String) -> String {
    let mut pname = String::from(DBDUMP_PATH);
    pname.push_str(&fname);
    pname
}

fn save_data(file: &mut File, data: String) -> Result<(), Error> {
    file.seek(SeekFrom::Start(0))?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn start_dbdump(app_info: &AppInfo) {
    let db = app_info.get_db_resolver();
    let fname = app_info.get_config_server().get_dumpfile();
    let mut file = File::create(generate_path(fname)).expect(ERROR_DBFILE_CREATE);
    let mut next_save: u64 = timestamp_now() + DBDUMP_INTERVAL_SECS;
    std::thread::spawn(move || loop {
        if next_save == timestamp_now() {
            let _ = save_data(&mut file, db.get_snapshot());
            next_save = timestamp_now() + DBDUMP_INTERVAL_SECS;
        }
    });
}
