use crate::{engine::{events::{event_manager::EventManager, game_event::GameEvent}, input::input::InputHandler, objects::{engine_animations::{ANIMATION_IDLE, ANIMATION_WALK}, game_object_controller::GameObjectController, game_object_core::GameObjectCore}, types::object_event::ObjectEvent}, game::entities::objects::projectiles::bullet::Bullet};
use crate::game::entities::factions::FACTION_PLAYER;
use crate::engine::utils::timer::Timer;
use std::f32::consts::PI;

pub struct PlayerInputController{
    fire_cooldown: Timer,
    shoot_direction: f32,
}

const HALF_PI: f32 = PI / 2.0;


impl PlayerInputController{
    pub fn new() -> PlayerInputController {
        return PlayerInputController { 
            fire_cooldown: Timer::new(500),
            shoot_direction: 0.0,
        };
    }
}


impl GameObjectController for PlayerInputController{
    fn update(&mut self, core: &mut GameObjectCore, _event: &ObjectEvent, input: &InputHandler, event_manager: &mut EventManager) {        
        if input.key_up(){
            core.movement_y = -1.0;
            self.shoot_direction = -HALF_PI;
        }

        if input.key_down(){
            core.movement_y = 1.0;
            self.shoot_direction = HALF_PI;
        }

        if input.key_left(){
            core.movement_x = -1.0;
            core.flip_sprite = true;
            self.shoot_direction = PI;
        }

        if input.key_right(){
            core.movement_x = 1.0;
            core.flip_sprite = false;
            self.shoot_direction = 0.0;
        }


        if core.x_velocity.abs() > 0.0 || core.y_velocity.abs() > 0.0 {
            core.play_animation(ANIMATION_WALK, false);
        }else {
            core.play_animation(ANIMATION_IDLE, false);
        }

        if input.key_action1() && self.fire_cooldown.can_activate() {
            let rust_x = core.x;
            let rust_y = core.y;
            let rust_direction = self.shoot_direction;
            self.fire_cooldown.activate();
            event_manager.push_event(GameEvent::SpawnObject { spawn_function: Box::new(move |game_object_manager|{
                game_object_manager.add_object(Bullet::new(rust_x, rust_y, rust_direction,"bow_0001", 4.0, FACTION_PLAYER, 10.0));
            })});
        }
    }
}