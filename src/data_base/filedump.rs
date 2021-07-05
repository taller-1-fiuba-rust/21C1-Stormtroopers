use crate::constants::{DBDUMP_INTERVAL_SECS, DBDUMP_PATH, ERROR_DBFILE_CREATE};
use crate::server::app_info::AppInfo;
use crate::server::utils::timestamp_now;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::{BufReader, SeekFrom};

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

pub fn start_filedump(app_info: &AppInfo) {
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

fn parse_line(line: String) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    for item in line.split('\t') {
        vec.push(item.to_string());
    }
    vec
}

pub fn load_filedump(app_info: &AppInfo) -> String {
    let db = app_info.get_db_resolver();
    let fname = app_info.get_config_server().get_dumpfile();
    let file = File::open(generate_path(fname)).expect(ERROR_DBFILE_CREATE);
    let lines = BufReader::new(file).lines();
    for line in lines.into_iter().flatten() {
        let parsed = parse_line(line);
        let dbname = &parsed[0];
        let key = &parsed[1];
        match dbname.as_str() {
            "String" => {
                let value = &parsed[2];
                db.get_string_db_sharding(key.as_str())
                    .set_string(key.clone(), value.clone());
            }
            "List" => {
                let mut args: Vec<&str> = parsed.iter().map(|s| s as &str).collect();
                args.remove(0);
                db.get_list_db_sharding(key.as_str()).lpush(args);
            }
            "Set" => {
                let mut args: Vec<&str> = parsed.iter().map(|s| s as &str).collect();
                args.remove(0);
                db.get_set_db_sharding(key.as_str()).sadd(args);
            }
            _ => {
                continue;
            }
        }
    }
    String::from("Data loaded!")
}
