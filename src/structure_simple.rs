use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct StructureSimple<'a> {
    pub structure: &'a mut Arc<Mutex<HashMap<String,String>>>,
}


    pub fn set_string(structure: & Arc<Mutex<HashMap<String,String>>>, key: String, value: String) {
        println!("structure_simple::set_string::{}::{}::", key,value);
        let mut data = structure.lock().unwrap();
        data.insert(key.clone(), value.clone());

        do_it(&mut data);
    }

    pub fn get_string(structure: & Arc<Mutex<HashMap<String,String>>>, key: String) -> String {
        println!("structure_simple::get_string::{}::", key);

        let mut data = structure.lock().unwrap();
        do_it(&mut data);

        match data.get_mut(&key) {
            Some(value) => {let mut response = String::from(value.clone()); response.push_str("\n"); response},
            None        => String::from("EMPTY_STRING\n"),
        }
    }

fn do_it(map: &mut HashMap<String, String>) {
    println!("look map");
    for (key, value) in &*map {
        println!("{} / {}", key, value);
    }
    //map.clear();
}