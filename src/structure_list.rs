use crate::structure_general::Storeable;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use std::any::Any;

pub struct StructureList<String> {
    structure: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl Default for StructureList<String> {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for StructureList<String> {
    fn clone(&self) -> Self {
        let structure = self.structure.clone();
        Self { structure }
    }
}

impl StructureList<String> {
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

impl Storeable for StructureList<String> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
