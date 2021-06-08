use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex};

pub struct DataBaseSet<String> {
    db_set: Arc<Mutex<HashMap<String, BTreeSet<String>>>>, //se puede cambiar por un HashSet,
                                                           //cuando se implementen los comandos se ve cuál es más fácil
}

impl Default for DataBaseSet<String> {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for DataBaseSet<String> {
    fn clone(&self) -> Self {
        let db_set = self.db_set.clone();
        Self { db_set }
    }
}

impl DataBaseSet<String> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let db_set = Arc::new(Mutex::new(HashMap::new()));
        Self { db_set }
    }

    #[allow(dead_code)]
    pub fn sadd(&self, key: String, value: String) {
        let mut db_set = self.db_set.lock().unwrap();

        let values = db_set.entry(key).or_insert_with(BTreeSet::<String>::new);
        values.insert(value);
    }

    pub fn clear_key(&self, key: String) {
        let mut db = self.db_set.lock().unwrap().clone();
        if db.contains_key(&key) {
            db.remove(&key);
        }
    }

    #[allow(dead_code)]
    pub fn get_set(&self, key: String) -> BTreeSet<String> {
        let db_set = self.db_set.lock().unwrap();
        db_set.get(&key).unwrap().clone()
    }

    #[allow(dead_code)]
    pub fn clean_all_data(&self) -> bool {
        let mut db_set = self.db_set.lock().unwrap();
        db_set.clear();
        db_set.is_empty()
    }

    #[allow(dead_code)]
    pub fn dbsize(&self) -> usize {
        let db_set = self.db_set.lock().unwrap();
        db_set.len()
    }
}
