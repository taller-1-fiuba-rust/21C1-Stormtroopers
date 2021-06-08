use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct DataBaseList<String> {
    structure: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl Default for DataBaseList<String> {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for DataBaseList<String> {
    fn clone(&self) -> Self {
        let structure = self.structure.clone();
        Self { structure }
    }
}

impl DataBaseList<String> {
    pub fn new() -> Self {
        let structure = Arc::new(Mutex::new(HashMap::new()));
        Self { structure }
    }

    #[allow(dead_code)]
    pub fn lpush(&self, key: String, value: String) {
        let mut structure = self.structure.lock().unwrap();

        let vec_values = structure.entry(key).or_insert_with(Vec::<String>::new);
        vec_values.push(value);
    }

    pub fn clear_key(&self, key: String) {
        let mut db = self.structure.lock().unwrap().clone();
        if db.contains_key(&key) {
            db.remove(&key);
        }
    }

    #[allow(dead_code)]
    pub fn get_list(&self, key: String) -> Vec<String> {
        let structure = self.structure.lock().unwrap();
        structure.get(&key).unwrap().clone()
    }

    #[allow(dead_code)]
    pub fn clean_all_data(&self) -> bool {
        let mut structure = self.structure.lock().unwrap();
        structure.clear();
        structure.is_empty()
    }

    #[allow(dead_code)]
    pub fn dbsize(&self) -> usize {
        let structure = self.structure.lock().unwrap();
        structure.len()
    }
}
