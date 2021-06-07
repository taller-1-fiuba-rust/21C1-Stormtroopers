use crate::logger::Loggable;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::thread;

pub struct TTLScheduler {
    ttl_map: HashMap<u64, String>,
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
        TTLScheduler {
            ttl_map: self.ttl_map.clone(),
        }
    }
}

impl TTLScheduler {
    pub fn new() -> TTLScheduler {
        TTLScheduler {
            ttl_map: HashMap::new(),
        }
    }
    pub fn run(&self) {
        for (key, value) in self.ttl_map.clone() {
            // check elapsed time and if 0 send message to structure to delete key value.
        }
    }
    pub fn add_ttl(ttl: u64, key: &str) {}
}
