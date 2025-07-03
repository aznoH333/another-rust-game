use crate::engine::objects::game_object_controller::GameObjectController;

pub struct DestroyOnHostileCollision{}

impl DestroyOnHostileCollision {
    pub fn new() -> DestroyOnHostileCollision {
        return DestroyOnHostileCollision {  }
    }
}

// TODO : rework to use an input array of hostile factions instead of assumptions about faction structure
impl GameObjectController for DestroyOnHostileCollision {
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, event: &crate::engine::types::object_event::ObjectEvent, input: &crate::engine::input::input::InputHandler, event_manager: &mut crate::engine::events::event_manager::EventManager) {
        let other = event.found_object.as_ref().unwrap();
        
        if other.faction != 0 && other.faction != core.faction {
            core.wants_to_live = false;
        }
    }
}