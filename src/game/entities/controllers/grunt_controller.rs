use crate::engine::objects::game_object_controller::GameObjectController;

pub struct FighterController {
    target_name: String
}

impl FighterController {
    pub fn new (target_name: &str) -> FighterController {
        return FighterController {
            target_name: target_name.to_owned()
        };
    }
}

impl GameObjectController for FighterController {
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, event: &crate::engine::types::object_event::ObjectEvent, input: &crate::engine::input::input::InputHandler, event_manager: &mut crate::engine::events::event_manager::EventManager) {
        
    }
}