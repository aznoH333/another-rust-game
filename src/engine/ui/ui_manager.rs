use crate::engine::ui::ui_values::UIValues;

pub struct UIManager{
    values: UIValues,
    // ui elements
}

impl UIManager {
    pub fn new() -> UIManager {
        return UIManager { 
            values: UIValues::new() 
        }
    }

    pub fn set_value(&mut self, key: &str, value: String) {
        self.values.set_value(key, value);
    }

    pub fn set_value_f32(&mut self, key: &str, value: f32) {
        self.values.set_value_f32(key, value);
    }
}
