use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct DataBaseList<String> {
    db_list: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl Default for DataBaseList<String> {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for DataBaseList<String> {
    fn clone(&self) -> Self {
        let db_list = self.db_list.clone();
        Self { db_list }
    }
}

impl DataBaseList<String> {
    pub fn new() -> Self {
        let db_list = Arc::new(Mutex::new(HashMap::new()));
        Self { db_list }
    }

    #[allow(dead_code)]
    pub fn lpush(&self, key: String, value: String) {
        let mut db_list = self.db_list.lock().unwrap();

        let vec_values = db_list.entry(key).or_insert_with(Vec::<String>::new);
        vec_values.push(value);
    }

    pub fn clear_key(&self, key: String) {
        let mut db = self.db_list.lock().unwrap().clone();
        if db.contains_key(&key) {
            db.remove(&key);
        }
    }

    #[allow(dead_code)]
    pub fn get_list(&self, key: String) -> Vec<String> {
        let db_list = self.db_list.lock().unwrap();
        db_list.get(&key).unwrap().clone()
    }

    #[allow(dead_code)]
    pub fn clean_all_data(&self) -> bool {
        let mut db_list = self.db_list.lock().unwrap();
        db_list.clear();
        db_list.is_empty()
    }

    #[allow(dead_code)]
    pub fn dbsize(&self) -> usize {
        let db_list = self.db_list.lock().unwrap();
        db_list.len()
    }
}
