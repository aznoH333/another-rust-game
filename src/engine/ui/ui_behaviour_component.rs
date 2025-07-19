use crate::engine::drawing::drawing_manager::DrawingManager;
use crate::engine::ui::ui_values::UIValues;

pub trait UIBehaviourComponent {
    fn draw(&self, drawing_manager: &mut DrawingManager, ui_values: &UIValues);
}