use crate::engine::objects::game_object_controller::GameObjectController;
use crate::engine::utils::timer::Timer;
pub struct TakeDamageOnHostileCollisionController{
    timer: Timer
}

impl TakeDamageOnHostileCollisionController {
    pub fn new(invulnerability_time: u128) -> TakeDamageOnHostileCollisionController {
        return TakeDamageOnHostileCollisionController {  
            timer: Timer::new(invulnerability_time),
        };
    }
}

impl GameObjectController for TakeDamageOnHostileCollisionController {
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, event: &crate::engine::types::object_event::ObjectEvent, input: &crate::engine::input::input::InputHandler, event_manager: &mut crate::engine::events::event_manager::EventManager) {
        let other = event.found_object.as_ref().unwrap();
        
        if other.faction != core.faction && 
        other.faction != 0 && 
        other.damage > 0.0 &&
        self.timer.can_activate(){
            self.timer.activate();
            core.health -= other.damage;
            if core.health <= 0.0 {
                core.wants_to_live = false;
            }
        }
    }
}