use crate::structure_general::Storeable;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex};

use std::any::Any;

pub struct StructureSet<String> {
    structure: Arc<Mutex<HashMap<String, BTreeSet<String>>>>, //se puede cambiar por un HashSet,
                                                              //cuando se implementen los comandos se ve cuál es más fácil
}

impl Default for StructureSet<String> {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for StructureSet<String> {
    fn clone(&self) -> Self {
        let structure = self.structure.clone();
        Self { structure }
    }
}

impl StructureSet<String> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let structure = Arc::new(Mutex::new(HashMap::new()));
        Self { structure }
    }

    #[allow(dead_code)]
    pub fn sadd(&self, key: String, value: String) {
        let mut structure = self.structure.lock().unwrap();

        let values = structure.entry(key).or_insert_with(BTreeSet::<String>::new);
        values.insert(value);
    }

    #[allow(dead_code)]
    pub fn get_set(&self, key: String) -> BTreeSet<String> {
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

impl Storeable for StructureSet<String> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
