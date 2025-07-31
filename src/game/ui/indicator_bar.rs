use ggez::graphics::Color;

use crate::engine::ui::ui_behaviour_component::UIBehaviourComponent;
use crate::game::enums::drawing_layers::DrawingLayer;
use crate::utils::number_utils::NumberUtils;

pub struct IndicatorBar {
    value: String,
    value_max: String,
    on_sprite: String,
    off_sprite: String,
    reverse: bool,
}

impl UIBehaviourComponent for IndicatorBar {
    fn draw(&self, drawing_manager: &mut crate::engine::drawing::drawing_manager::DrawingManager, ui_values: &crate::engine::ui::ui_values::UIValues, x: f32, y: f32) {
        let value = ui_values.get_value_f32(&self.value) as i32;
        let max_value = ui_values.get_value_f32(&self.value_max) as i32;
        for i in 0..max_value {

            let sprite = if (i + 1) <= value { &self.on_sprite } else { &self.off_sprite };

            drawing_manager.draw_sprite(sprite, 
                x + (i * 8 * -NumberUtils::bool_to_minus_plus(self.reverse)) as f32, 
                y, 
                DrawingLayer::UIFront.get_value(), 
                1.0, 
                false, 
                0.0, 
                Color::WHITE, 
                true);
        }
    }
}

impl IndicatorBar {
    pub fn new(value: &str, value_max: &str, on_sprite: &str, off_sprite: &str, reverse: bool) -> IndicatorBar {
        return IndicatorBar { 
            value: value.to_string(), 
            value_max: value_max.to_string(), 
            on_sprite: on_sprite.to_string(), 
            off_sprite: off_sprite.to_string(),
            reverse: reverse
        }
    }
}