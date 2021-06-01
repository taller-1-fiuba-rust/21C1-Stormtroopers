/*
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct StructureString<'a> {
    structure: &'a mut Arc<Mutex<HashMap<String,String>>>,
}

impl <'a>StructureString<'a> {
    pub fn new(structure: &'a mut Arc<Mutex<HashMap<String,String>>>) -> StructureString<'a> {

        StructureString {
            structure,
        }
    }

    pub fn set_string(&self,key: String, value: String) {
        /*
        let mut key_val_sender = key;
        key_val_sender.push_str(":");
        key_val_sender.push_str(&value);
         */

        let mut data = self.structure.lock().unwrap();
        data.insert(key.clone(), value.clone());

        //do_it(&mut data);
    }

    pub fn get_string(&self, key: String) -> String {
        println!("structure_string::get_string::{}::", key);
        //do_it(&mut self.structure.lock().unwrap());

        let mut data = self.structure.lock().unwrap();
        do_it(&mut data);

        match data.get_mut(&key) {
            Some(value) => String::from(value.clone()),
            None        => String::from("EMPTY_STRING\n"),
        }
    }
}

fn do_it(map: &mut HashMap<String, String>) {
    println!("look map");
    for (key, value) in &*map {
        println!("{} / {}", key, value);
    }
    //map.clear();
}
 */