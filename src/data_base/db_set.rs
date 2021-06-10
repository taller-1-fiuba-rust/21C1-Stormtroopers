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
    pub fn sadd(&self, mut args: Vec<&str>) -> u32 {
        let mut db_set = self.db_set.lock().unwrap();
        let key = args.remove(0);

        let set_st = db_set
            .entry(String::from(key))
            .or_insert_with(BTreeSet::<String>::new);
        let mut insertion;
        let mut i = 0_u32;
        for val in args {
            insertion = set_st.insert(val.to_string());
            if insertion {
                i += 1;
            }
        }
        i
    }

    pub fn smembers(&self, mut args: Vec<&str>) -> Vec<String> {
        let db_set = self.db_set.lock().unwrap();
        let key = args.remove(0);
        let set_st = match db_set.get(&key.to_string()) {
            Some(set) => set,
            None => return vec![],
        };

        let mut values: Vec<String> = vec![];
        for val in set_st.iter() {
            values.push(val.clone());
        }
        values
    }

    pub fn scard(&self, mut args: Vec<&str>) -> u32 {
        let db_set = self.db_set.lock().unwrap();
        let key = args.remove(0);

        return match db_set.get(&key.to_string()) {
            Some(set) => set.len() as u32,
            None => 0_u32,
        };
    }

    pub fn sismember(&self, args: Vec<&str>) -> u8 {
        let db_set = self.db_set.lock().unwrap();
        let key = args[0];
        let value = args[1];

        return match db_set.get(&key.to_string()) {
            Some(set) => match set.get(value) {
                Some(_v) => 1,
                None => 0,
            },
            None => return 0,
        };
    }

    pub fn srem(&self, mut args: Vec<&str>) -> u32 {
        let mut db_set = self.db_set.lock().unwrap();
        let key = args.remove(0);

        let set_st = db_set
            .entry(String::from(key))
            .or_insert_with(BTreeSet::<String>::new);
        let mut rem;
        let mut i = 0_u32;
        for val in args {
            rem = set_st.remove(&val.to_string());
            if rem {
                i += 1;
            }
        }
        i
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
