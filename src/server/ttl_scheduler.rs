use crate::errors::run_error::RunError;
use crate::server::logger::Loggable;
use crate::server::utils;
use crate::AppInfo;
use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const TTL_CHECK_RANGE: u64 = 5;
const NOT_FOUND: &str = "Key not found.";
const OK: &str = "Ok";

pub struct TTLScheduler {
    pub ttl_map: Arc<Mutex<HashMap<u64, String>>>,
    pub helper_map: Arc<Mutex<HashMap<String, u64>>>,
    sender: Arc<SyncSender<String>>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl Loggable for TTLScheduler {
    fn get_id_client(&self) -> &str {
        "TTLScheduler"
    }
    fn get_id_thread(&self) -> u32 {
        0_u32
    }
}

impl Clone for TTLScheduler {
    fn clone(&self) -> TTLScheduler {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        let ttl_map = self.ttl_map.clone();
        let helper_map = self.helper_map.clone();
        Self {
            ttl_map,
            helper_map,
            sender,
            receiver,
        }
    }
}

impl Default for TTLScheduler {
    fn default() -> Self {
        TTLScheduler::new()
    }
}

impl TTLScheduler {
    pub fn new() -> TTLScheduler {
        let ttl_map = Arc::new(Mutex::new(HashMap::new()));
        let helper_map = Arc::new(Mutex::new(HashMap::new()));
        let (tx, rx) = sync_channel(1);
        let sender = Arc::new(tx);
        let receiver = Arc::new(Mutex::new(rx));
        TTLScheduler {
            ttl_map: ttl_map,
            helper_map: helper_map,
            sender: sender,
            receiver: receiver,
        }
    }

    pub fn run(&mut self, app_info: &AppInfo) {
        let ttl_scheduler = self.clone();
        let db = app_info.get_db_resolver().clone();
        thread::spawn(move || loop {
            let now: u64 = utils::timestamp_now();
            thread::sleep(Duration::from_secs(1));
            for n in (0..TTL_CHECK_RANGE).rev() {
                let time = now - n;
                let time_str = time.to_string();
                match ttl_scheduler.delete_ttl(time_str.clone()) {
                    Ok(key) => {
                        let _aux = ttl_scheduler.delete_ttl_helper(key.clone());

                        match db.type_key(key.clone()) {
                            Ok(val) => match val.as_str() {
                                "String" => {
                                    db.get_string_db().delete(vec![key.as_str()]);
                                }
                                "List" => {
                                    db.get_list_db().clear_key(key);
                                }
                                "Set" => {
                                    db.get_set_db().clear_key(key);
                                }
                                _ => (),
                            },
                            Err(_) => (),
                        }
                    }
                    Err(_) => continue,
                }
            }
        });
    }

    pub fn set_ttl(&self, ttl: u64, arg: String) -> Result<String, RunError> {
        match self.set_helper_ttl(ttl.clone(), arg.clone()) {
            Ok(_) => {
                let mut key_val = ttl.to_string();
                key_val.push(':');
                key_val.push_str(&arg);

                let mut ttl_scheduler = self.clone();
                let mut ttl_map = self.ttl_map.clone();

                return thread::spawn(move || {
                    ttl_scheduler.sender.send(key_val).unwrap();
                    ttl_scheduler.store(&mut ttl_map)
                })
                .join()
                .unwrap();
            }
            Err(e) => Err(e),
        }
    }

    fn set_helper_ttl(&self, ttl: u64, mut arg: String) -> Result<String, RunError> {
        arg.push(':');
        arg.push_str(ttl.to_string().as_str());

        let mut ttl_scheduler = self.clone();
        let mut helper_map = self.helper_map.clone();

        return thread::spawn(move || {
            ttl_scheduler.sender.send(arg).unwrap();
            ttl_scheduler.store_helper(&mut helper_map)
        })
        .join()
        .unwrap();
    }

    /*fn get_ttl(&self, arg: String) -> Result<String, RunError> {
        let mut ttl_scheduler = self.clone();
        let mut ttl_map = self.ttl_map.clone();

        return thread::spawn(move || {
            ttl_scheduler.sender.send(arg).unwrap();
            ttl_scheduler.retrieve(&mut ttl_map)
        })
        .join()
        .unwrap();
    }*/

    pub fn get_ttl_helper(&self, arg: String) -> Result<String, RunError> {
        let mut ttl_scheduler = self.clone();
        let mut ttl_map = self.helper_map.clone();

        return thread::spawn(move || {
            ttl_scheduler.sender.send(arg).unwrap();
            ttl_scheduler.retrieve_helper(&mut ttl_map)
        })
        .join()
        .unwrap();
    }

    pub fn delete_ttl(&self, arg: String) -> Result<String, String> {
        let mut ttl_scheduler = self.clone();
        let mut ttl_map = self.ttl_map.clone();

        thread::spawn(move || {
            ttl_scheduler.sender.send(arg).unwrap();
            ttl_scheduler.delete(&mut ttl_map)
        })
        .join()
        .unwrap()
    }
    pub fn delete_ttl_helper(&self, arg: String) -> Result<String, String> {
        let mut ttl_scheduler = self.clone();
        let mut ttl_map = self.helper_map.clone();

        thread::spawn(move || {
            ttl_scheduler.sender.send(arg).unwrap();
            ttl_scheduler.delete_helper(&mut ttl_map)
        })
        .join()
        .unwrap()
    }

    fn store(&mut self, map: &mut Arc<Mutex<HashMap<u64, String>>>) -> Result<String, RunError> {
        let key_val = self.receiver.lock().unwrap().recv().unwrap();
        let kv_splitted: Vec<&str> = key_val.split(':').collect();

        let mut map = map.lock().unwrap();
        map.insert(
            kv_splitted[0].trim().parse::<u64>().unwrap(),
            String::from(kv_splitted[1].trim()),
        );
        Ok(String::from(OK))
    }

    fn store_helper(
        &mut self,
        map: &mut Arc<Mutex<HashMap<String, u64>>>,
    ) -> Result<String, RunError> {
        let key_val = self.receiver.lock().unwrap().recv().unwrap();
        let kv_splitted: Vec<&str> = key_val.split(':').collect();

        let mut map = map.lock().unwrap();
        map.insert(
            String::from(kv_splitted[0].trim()),
            kv_splitted[1].trim().parse::<u64>().unwrap(),
        );
        Ok(String::from(OK))
    }

    /*fn retrieve(&mut self, map: &mut Arc<Mutex<HashMap<u64, String>>>) -> Result<String, RunError> {
        let key = self.receiver.lock().unwrap().recv().unwrap();
        let key_parsed = key.parse::<u64>().unwrap();

        let map = map.lock().unwrap();
        match map.get(&key_parsed) {
            Some(value) => Ok(value.clone()),
            None => Err(RunError{message: key, cause: String::from(NOT_FOUND)})
        }
    }*/

    fn retrieve_helper(
        &mut self,
        map: &mut Arc<Mutex<HashMap<String, u64>>>,
    ) -> Result<String, RunError> {
        let key = self.receiver.lock().unwrap().recv().unwrap();

        let map = map.lock().unwrap();

        match map.get(&key) {
            Some(ttl) => Ok(ttl.to_string()),
            None => Err(RunError {
                message: key,
                cause: String::from(NOT_FOUND),
            }),
        }
    }

    fn delete(&mut self, map: &mut Arc<Mutex<HashMap<u64, String>>>) -> Result<String, String> {
        let key = self.receiver.lock().unwrap().recv().unwrap();
        let key_parsed = key.parse::<u64>().unwrap();
        let mut map = map.lock().unwrap();
        match map.remove(&key_parsed) {
            Some(v) => Ok(v),
            None => Err(String::from("")),
        }
    }

    fn delete_helper(
        &mut self,
        map: &mut Arc<Mutex<HashMap<String, u64>>>,
    ) -> Result<String, String> {
        let key = self.receiver.lock().unwrap().recv().unwrap();
        let mut map = map.lock().unwrap();
        match map.remove(&key) {
            Some(v) => Ok(v.to_string()),
            None => Err(String::from("")),
        }
    }
}
