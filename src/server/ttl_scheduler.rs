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

pub struct TtlScheduler {
    pub ttl_map: Arc<Mutex<HashMap<u64, String>>>,
    pub key_map: Arc<Mutex<HashMap<String, u64>>>,
    sender: Arc<SyncSender<String>>,
    receiver: Arc<Mutex<Receiver<String>>>,
}

impl Loggable for TtlScheduler {
    fn get_id_client(&self) -> &str {
        "TTLScheduler"
    }
    fn get_id_thread(&self) -> u32 {
        0_u32
    }
}

impl Clone for TtlScheduler {
    fn clone(&self) -> TtlScheduler {
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        let ttl_map = self.ttl_map.clone();
        let key_map = self.key_map.clone();
        Self {
            ttl_map,
            key_map,
            sender,
            receiver,
        }
    }
}

impl Default for TtlScheduler {
    fn default() -> Self {
        TtlScheduler::new()
    }
}

impl TtlScheduler {
    pub fn new() -> TtlScheduler {
        let ttl_map = Arc::new(Mutex::new(HashMap::new()));
        let key_map = Arc::new(Mutex::new(HashMap::new()));
        let (tx, rx) = sync_channel(1);
        let sender = Arc::new(tx);
        let receiver = Arc::new(Mutex::new(rx));
        TtlScheduler {
            ttl_map,
            key_map,
            sender,
            receiver,
        }
    }

    pub fn run(&mut self, app_info: &AppInfo) {
        let ttl_scheduler = self.clone();
        let db = app_info.get_db_resolver();
        thread::spawn(move || loop {
            let now: u64 = utils::timestamp_now();
            thread::sleep(Duration::from_secs(1));
            for n in (0..TTL_CHECK_RANGE).rev() {
                let time = now - n;
                let time_str = time.to_string();
                match ttl_scheduler.delete_ttl(time_str.clone()) {
                    Ok(key) => {
                        let _aux = ttl_scheduler.delete_ttl_key(key.clone());

                        if let Ok(val) = db.type_key(key.clone()) {
                            match val.as_str() {
                                "String" => {
                                    //db.get_string_db().delete(vec![key.as_str()]);
                                    db.get_string_db_sharding(key.as_str())
                                        .delete(vec![key.as_str()]);
                                }
                                "List" => {
                                    db.get_list_db_sharding(key.as_str()).clear_key(key);
                                }
                                "Set" => {
                                    db.get_set_db_sharding(key.as_str()).clear_key(key);
                                }
                                _ => (),
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }
        });
    }

    pub fn set_ttl(&self, ttl: u64, arg: String) -> Result<String, RunError> {
        match (self.set_key_ttl(ttl, arg.clone())).as_str() {
            "" => (),
            old_ttl => {self.delete_ttl(String::from(old_ttl));}
        };

        let mut key_val = ttl.to_string();
        key_val.push(':');
        key_val.push_str(&arg);

        let mut ttl_scheduler = self.clone();
        let mut ttl_map = self.ttl_map.clone();

        let result = thread::spawn(move || {
            ttl_scheduler.sender.send(key_val).unwrap();
            ttl_scheduler.store(&mut ttl_map)
        })
        .join()
        .unwrap();
        Ok(result)
    }

    fn set_key_ttl(&self, ttl: u64, mut arg: String) -> String {
        arg.push(':');
        arg.push_str(ttl.to_string().as_str());

        let mut ttl_scheduler = self.clone();
        let mut key_map = self.key_map.clone();

        let result = thread::spawn(move || {
            ttl_scheduler.sender.send(arg).unwrap();
            ttl_scheduler.store_key(&mut key_map)
        })
        .join()
        .unwrap();
        result
    }

    pub fn get_ttl_key(&self, arg: String) -> Result<String, RunError> {
        let mut ttl_scheduler = self.clone();
        let mut ttl_map = self.key_map.clone();

        return thread::spawn(move || {
            ttl_scheduler.sender.send(arg).unwrap();
            ttl_scheduler.retrieve_key(&mut ttl_map)
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
    pub fn delete_ttl_key(&self, arg: String) -> Result<String, String> {
        let mut ttl_scheduler = self.clone();
        let mut ttl_map = self.key_map.clone();

        thread::spawn(move || {
            ttl_scheduler.sender.send(arg).unwrap();
            ttl_scheduler.delete_key(&mut ttl_map)
        })
        .join()
        .unwrap()
    }

    fn store(&mut self, map: &mut Arc<Mutex<HashMap<u64, String>>>) -> String {
        let key_val = self.receiver.lock().unwrap().recv().unwrap();
        let kv_splitted: Vec<&str> = key_val.split(':').collect();

        let mut map = map.lock().unwrap();
        map.insert(
            kv_splitted[0].trim().parse::<u64>().unwrap(),
            String::from(kv_splitted[1].trim()),
        );
        String::from(OK)
    }

    fn store_key(&mut self, map: &mut Arc<Mutex<HashMap<String, u64>>>) -> String {
        let key_val = self.receiver.lock().unwrap().recv().unwrap();
        let kv_splitted: Vec<&str> = key_val.split(':').collect();

        let mut map = map.lock().unwrap();
        match map.insert(
            String::from(kv_splitted[0].trim()),
            kv_splitted[1].trim().parse::<u64>().unwrap(),
        ) {
            Some(value) => value.to_string(),
            None => String::from("")
        }
    }

    fn retrieve_key(
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

    fn delete_key(&mut self, map: &mut Arc<Mutex<HashMap<String, u64>>>) -> Result<String, String> {
        let key = self.receiver.lock().unwrap().recv().unwrap();
        let mut map = map.lock().unwrap();
        match map.remove(&key) {
            Some(v) => Ok(v.to_string()),
            None => Err(String::from("")),
        }
    }
}
