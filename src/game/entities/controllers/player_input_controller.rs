use crate::engine::{events::{event_manager, game_event::GameEvent}, objects::{game_object_controller::GameObjectController, game_object_core::GameObjectCore, object_summon::ObjectSummon, object_update::ObjectUpdate}};
use crate::game::entities::factions::FACTION_PLAYER;
use crate::engine::utils::timer::Timer;
use std::f32::consts::PI;

pub struct PlayerInputController{
    shoot_direction: f32,
}

const HALF_PI: f32 = PI / 2.0;


impl PlayerInputController{
    pub fn new() -> PlayerInputController {
        return PlayerInputController { 
            shoot_direction: 0.0,
        };
    }
}


impl GameObjectController for PlayerInputController{
    fn update(&mut self, core: &mut GameObjectCore, update_value: &mut ObjectUpdate) {        
        if update_value.input.key_up(){
            core.movement_y = -1.0;
            self.shoot_direction = -HALF_PI;
        }

        if update_value.input.key_down(){
            core.movement_y = 1.0;
            self.shoot_direction = HALF_PI;
        }

        if update_value.input.key_left(){
            core.movement_x = -1.0;
            core.flip_sprite = true;
            self.shoot_direction = PI;
        }

        if update_value.input.key_right(){
            core.movement_x = 1.0;
            core.flip_sprite = false;
            self.shoot_direction = 0.0;
        }

        core.set_weapon_direction(self.shoot_direction);
        

        if update_value.input.key_action1() {
            core.attack(update_value.event_manager);
        }
    }
}