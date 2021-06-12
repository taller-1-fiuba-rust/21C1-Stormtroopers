use crate::logger::Loggable;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{SyncSender, Receiver, sync_channel};
use crate::errors::run_error::RunError;

const TTL_CHECK_RANGE: u64 = 10;
const RESPONSE_NIL: &str = "(nil)";

pub struct TTLScheduler {
    pub ttl_map: Arc<Mutex<HashMap<u64, String>>>,
    pub helper_map: Arc<Mutex<HashMap<String, u64>>>,
    sender: Arc<SyncSender<String>>,
    receiver: Arc<Mutex<Receiver<String>>>
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
            receiver: receiver
        }
    }

    pub fn run(&mut self) {
        std::thread::spawn(move || {
            loop {
            }
        });
    }

    // ttl is the timestamp, args is a string with the key
    pub fn set_ttl(&self, ttl: u64, arg: String) -> Result<String, RunError> {
        self.set_helper_ttl(ttl.clone(), arg.clone());
        let mut key_val = ttl.to_string();
        key_val.push(':');
        key_val.push_str(&arg);

        let mut ttl_scheduler = self.clone();
        let mut ttl_map = self.ttl_map.clone();

        let ttl_h = std::thread::spawn(move || {
            ttl_scheduler.sender.send(key_val).unwrap();
            ttl_scheduler.store(&mut ttl_map)
        }).join().unwrap();

        Ok(ttl_h)
    }

    pub fn set_helper_ttl(&self, ttl: u64, mut arg: String) {
        arg.push(':');
        arg.push_str(ttl.to_string().as_str());

        let mut ttl_scheduler = self.clone();
        let mut helper_map = self.helper_map.clone();

        std::thread::spawn(move || {
            ttl_scheduler.sender.send(arg).unwrap();
            ttl_scheduler.store_helper(&mut helper_map)
        }).join().unwrap();
    }
    
    pub fn get_ttl(&self, arg: String) -> Result<String, RunError> {
        let mut ttl_scheduler = self.clone();
        let mut ttl_map = self.helper_map.clone();

        let handle = std::thread::spawn(move || {
            ttl_scheduler.sender.send(arg).unwrap();
            ttl_scheduler.retrieve(&mut ttl_map)
        })
        .join()
        .unwrap();

        Ok(handle)
    }

    pub fn store(&mut self, map: &mut Arc<Mutex<HashMap<u64, String>>>) -> String {
        let key_val = self.receiver.lock().unwrap().recv().unwrap();
        let kv_splitted: Vec<&str> = key_val.split(':').collect();

        let mut map = map.lock().unwrap();
        map.insert(
            kv_splitted[0].trim().parse::<u64>().unwrap(),
            String::from(kv_splitted[1].trim())
        );
        String::from("Ok")
    }

    pub fn store_helper(&mut self, map: &mut Arc<Mutex<HashMap<String, u64>>>) {
        let key_val = self.receiver.lock().unwrap().recv().unwrap();
        let kv_splitted: Vec<&str> = key_val.split(':').collect();

        let mut map = map.lock().unwrap();
        map.insert(
            String::from(kv_splitted[0].trim()),
            kv_splitted[1].trim().parse::<u64>().unwrap(),
        );
    }

    pub fn retrieve(&mut self, map: &mut Arc<Mutex<HashMap<String, u64>>>) -> String {
        let value = self.receiver.lock().unwrap().recv().unwrap();

        let map_t = map.lock().unwrap();

        match map_t.get(&value) {
            Some(ttl) => ttl.to_string(),
            None => String::from(RESPONSE_NIL)
        }
    }
}
