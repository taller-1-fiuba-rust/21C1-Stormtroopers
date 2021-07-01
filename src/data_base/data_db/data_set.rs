use std::collections::BTreeSet;
use std::time::SystemTime;

pub struct DataSet<String> {
    value: BTreeSet<String>,
    time_touch: SystemTime,
}

impl Clone for DataSet<String> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            time_touch: self.time_touch,
        }
    }
}

impl DataSet<String> {
    pub fn new() -> Self {
        Self {
            value: BTreeSet::<String>::new(),
            time_touch: SystemTime::now(),
        }
    }

    pub fn get_value(&self) -> BTreeSet<String> {
        self.value.clone()
    }

    #[allow(dead_code)]
    pub fn get_time_touch(&self) -> SystemTime {
        self.time_touch
    }

    pub fn insert_value(&mut self, value: String) -> bool {
        if self.value.insert(value) {
            self.time_touch = SystemTime::now();
            return true;
        }
        false
    }

    pub fn update_touch(&mut self) {
        self.time_touch = SystemTime::now();
    }

    pub fn remove_value(&mut self, value: String) -> bool {
        if self.value.remove(&value) {
            self.time_touch = SystemTime::now();
            return true;
        }
        false
    }
}
