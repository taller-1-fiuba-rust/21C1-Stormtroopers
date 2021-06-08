use crate::logger::Loggable;
use std::collections::HashMap;
use std::time::{UNIX_EPOCH, SystemTime};

// The key is the timestamp from EPOCH to expiration time. //751840123850917
// <ttl, [[string, clave1],[data_tipe2, clave2],...]>
pub struct TTLScheduler {
    ttl_map: HashMap<u64, Vec<(String, String)>>,
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

    pub fn run(&mut self) {
        let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
        // Revisar menor o igual en las claves y recorrer el hashmap.
        // Podriamos ingresar al hashmap con un rango desde el timestamp, si es 100, que revise, por ejemplo del 97 al 100.
        // Podria ser una lista, dado que hay que recorrer las claves por <= al timestamp del system.now
        // Deberiamos tener una lista ordenada!
        if self.ttl_map.get(&now).is_some() {
            // llamar a funcion que elimine los valores en StructureGeneral
            self.ttl_map.remove(&now);
        }
    }

    pub fn add_ttl(&mut self, ttl: u64, key: &str) {
        self.ttl_map.insert(ttl, vec!((String::from("String"), String::from(key))));
    }
}

impl Default for TTLScheduler {
    fn default() -> Self {
        TTLScheduler::new()
    }
}
