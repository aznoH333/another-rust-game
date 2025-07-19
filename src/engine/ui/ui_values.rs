use std::collections::HashMap;

pub struct UIValues {
    values: HashMap<String, String>,
}

impl UIValues {
    pub fn new() -> UIValues {
        return UIValues {
            values: HashMap::new()
        };
    }

    pub fn set_value(&mut self, key: &str, value: String) {
        self.values.insert(key.to_string(), value);
    }

    pub fn set_value_f32(&mut self, key: &str, value: f32) {
        self.values.insert(key.to_string(), value.to_string());
    }

    pub fn get_value(&self, key: &str) -> &String {
        return self.values.get(key).expect(format!("value with key not found {}", key).as_str());
    }

    pub fn get_value_f32(&self, key: &str) -> f32 {
        let value = self.get_value(key);
        return value.parse::<f32>().expect(format!("could not convert value {} to f32 (key {})", value, key).as_str())
    }
}