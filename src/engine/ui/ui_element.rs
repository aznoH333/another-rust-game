use crate::engine::drawing::drawing_manager::DrawingManager;
use crate::engine::objects::drawable::game_sprite::GameSprite;
use crate::engine::ui::ui_behaviour_component::UIBehaviourComponent;
use crate::engine::ui::ui_values::UIValues;

pub struct UIElement {
    sprite: GameSprite,
    visible: bool,
    children: Vec<UIElement>,
    component: Option<Box<dyn UIBehaviourComponent>>,
}

impl UIElement {
    pub fn new(x: f32, y: f32, sprite: &str, z_index: i32) -> UIElement {
        return UIElement { 
            sprite: GameSprite::new(x, y, sprite, z_index), 
            visible: true, 
            children: Vec::new(),
            component: None,
        }
    }

    pub fn draw(&self, drawing_manager: &mut DrawingManager, ui_values: &UIValues) {
        if !self.visible {
            return;
        }

        self.sprite.draw(drawing_manager);

        if self.component.is_some() {
            self.component.as_ref().unwrap().draw(drawing_manager, ui_values);
        }

        // children
        for child in &self.children {
            child.draw(drawing_manager, ui_values);
        }
    }

    pub fn set_dimension(mut self, width: f32, height: f32) -> UIElement {
        self.sprite.set_width(width);
        self.sprite.set_height(height);
        return self;
    }

    pub fn set_visibility(&mut self, visibility: bool) {
        self.visible = visibility;
    }
}