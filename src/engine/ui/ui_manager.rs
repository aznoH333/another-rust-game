use std::collections::HashMap;

use crate::engine::drawing::drawing_manager;
use crate::engine::drawing::drawing_manager::DrawingManager;
use crate::engine::ui::ui_element::UIElement;
use crate::engine::ui::ui_values::UIValues;

pub struct UIManager{
    values: UIValues,
    ui_groups: HashMap<String, UIElement>
}

impl UIManager {
    pub fn new() -> UIManager {
        return UIManager { 
            values: UIValues::new(),
            ui_groups: HashMap::new(),
        }
    }

    pub fn set_value(&mut self, key: &str, value: String) {
        self.values.set_value(key, value);
    }

    pub fn set_value_f32(&mut self, key: &str, value: f32) {
        self.values.set_value_f32(key, value);
    }

    pub fn add_ui_group(&mut self, group_name: &str, group: UIElement) {
        self.ui_groups.insert(group_name.to_string(), group);
    }

    pub fn set_group_visibility(&mut self, group_name: &str, value: bool) {
        self.ui_groups.get_mut(group_name).unwrap().set_visibility(value);
    }

    pub fn draw(&self, drawing_manager: &mut DrawingManager) {
        for group in self.ui_groups.values() {
            group.draw(drawing_manager, &self.values);
        }
    }
}
