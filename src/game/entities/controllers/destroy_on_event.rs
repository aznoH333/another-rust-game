use crate::engine::objects::game_object_controller::GameObjectController;

pub struct DestroyOnEvent{

}

impl DestroyOnEvent {
    pub fn new() -> DestroyOnEvent {
        return DestroyOnEvent {  };
    }
}

impl GameObjectController for DestroyOnEvent {
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, event: &crate::engine::types::object_event::ObjectEvent, input: &crate::engine::input::input::InputHandler, event_manager: &mut crate::engine::events::event_manager::EventManager) {
        core.die();
    }
}