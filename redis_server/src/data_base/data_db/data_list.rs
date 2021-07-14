use std::time::SystemTime;

pub struct DataList<String> {
    value: Vec<String>,
    time_touch: SystemTime,
}

impl Clone for DataList<String> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            time_touch: self.time_touch,
        }
    }
}

impl DataList<String> {
    pub fn new() -> Self {
        Self {
            value: Vec::<String>::new(),
            time_touch: SystemTime::now(),
        }
    }

    pub fn get_value(&self) -> Vec<String> {
        self.value.clone()
    }

    pub fn insert_values(&mut self, values: Vec<String>) {
        for value in values {
            self.value.push(value);
        }
        self.time_touch = SystemTime::now();
    }

    pub fn insert_value(&mut self, value: String) {
        self.value.push(value);
        self.time_touch = SystemTime::now();
    }

    pub fn update_touch(&mut self) -> u64 {
        let old_time = self.time_touch;
        self.time_touch = SystemTime::now();
        self.time_touch.duration_since(old_time).unwrap().as_secs()
    }
}
