/*
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

const RESPONSE_EMPTY_STRING: &str = "Empty String\n";

pub struct StructureSimple<'a> {
    pub structure: &'a mut Arc<Mutex<HashMap<String,String>>>,
}

    pub fn set_string(structure: & Arc<Mutex<HashMap<String,String>>>, key: String, value: String) {
        let mut data = structure.lock().unwrap();
        data.insert(key.clone(), value.clone());
    }

    pub fn get_string(structure: & Arc<Mutex<HashMap<String,String>>>, key: String) -> String {
        let mut data = structure.lock().unwrap();

        match data.get_mut(&key) {
            Some(value) => {let mut response = String::from(value.clone()); response.push_str("\n"); response},
            None        => String::from(RESPONSE_EMPTY_STRING),
        }
    }
 */