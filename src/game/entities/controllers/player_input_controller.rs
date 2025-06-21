use crate::{engine::{events::{event_manager::{self, EventManager}, game_event::GameEvent}, input::input::InputHandler, objects::{game_object_controller::GameObjectController, game_object_core::GameObjectCore}}, game::entities::objects::projectiles::bullet::Bullet};

pub struct PlayerInputController{

}


impl PlayerInputController{
    pub fn new() -> PlayerInputController {
        return PlayerInputController {  };
    }
}


impl GameObjectController for PlayerInputController{
    fn update(&mut self, core: &mut GameObjectCore, input: &InputHandler, event_manager: &mut EventManager) {
        if input.key_up(){
            core.y_velocity = -1.0;
        }

        if input.key_down(){
            core.y_velocity = 1.0;
        }

        if input.key_left(){
            core.x_velocity = -1.0;
        }

        if input.key_right(){
            core.x_velocity = 1.0;
        }


        if input.key_action1() {
            let rust_x = core.x;
            let rust_y = core.y;
            event_manager.push_event(GameEvent::SpawnObject { spawn_function: Box::new(move |game_object_manager|{
                game_object_manager.add_object(Bullet::new(rust_x, rust_y, 0.0,"bow_0001", 4.0));
            })});
        }
    }
    
    fn on_world_collide(&mut self, core: &mut GameObjectCore, event_manager: &mut EventManager) {
        // do nothing
    }
}