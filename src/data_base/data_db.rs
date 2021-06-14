use std::time::SystemTime;

pub struct Data<String> {
    value: Vec<String>,
    time_touch: SystemTime,
}

impl Clone for Data<String> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            time_touch: self.time_touch,
        }
    }
}

impl Data<String> {
    pub fn new() -> Self {
        Self {
            value: Vec::<String>::new(),
            time_touch: SystemTime::now(),
        }
    }

    pub fn get_value(&self) -> Vec<String> {
        self.value.clone()
    }

    #[allow(dead_code)]
    pub fn get_time_touch(&self) -> SystemTime {
        self.time_touch
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

    pub fn update_touch(&mut self) {
        self.time_touch = SystemTime::now();
    }
}
