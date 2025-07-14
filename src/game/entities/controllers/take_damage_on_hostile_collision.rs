use crate::engine::objects::game_object_controller::GameObjectController;
use crate::engine::objects::object_update::ObjectUpdate;
use crate::engine::utils::timer::Timer;
use crate::utils::space_utils::SpaceUtils;
pub struct TakeDamageOnHostileCollisionController{
    timer: Timer,
    knockback_multiplier: f32,
}

impl TakeDamageOnHostileCollisionController {
    pub fn new(invulnerability_time: u128, knockback_multiplier: f32) -> TakeDamageOnHostileCollisionController {
        return TakeDamageOnHostileCollisionController {  
            timer: Timer::new(invulnerability_time),
            knockback_multiplier
        };
    }
}

impl GameObjectController for TakeDamageOnHostileCollisionController {
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, update: &mut ObjectUpdate) {
        let other = update.event.found_object.as_ref().unwrap();
        
        if other.faction != core.faction && 
        other.faction != 0 && 
        other.damage > 0.0 &&
        self.timer.can_activate(){
            self.timer.activate();
            core.health -= other.damage;
            core.stun_timer.activate();
            if core.health <= 0.0 {
                core.die();
            }

            // knockback
            let direction = SpaceUtils::direction_towards(other.position.x, other.position.y, core.get_x(), core.get_y());
            core.x_velocity += direction.cos() * self.knockback_multiplier;
            core.y_velocity += direction.sin() * self.knockback_multiplier;
        }
    }
}