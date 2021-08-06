//! String data type.
use std::time::SystemTime;

pub struct DataString<String> {
    value: String,
    time_touch: SystemTime,
}

impl Clone for DataString<String> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            time_touch: self.time_touch,
        }
    }
}

impl DataString<String> {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            time_touch: SystemTime::now(),
        }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }

    pub fn update_touch(&mut self) -> u64 {
        let old_time = self.time_touch;
        self.time_touch = SystemTime::now();
        self.time_touch.duration_since(old_time).unwrap().as_secs()
    }

    pub fn insert_value(&mut self, value: String) {
        self.value = value;
        self.time_touch = SystemTime::now();
    }
}
