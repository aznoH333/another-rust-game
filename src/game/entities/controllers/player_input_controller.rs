use crate::engine::{input::input::InputHandler, objects::{game_object_controller::GameObjectController, game_object_core::GameObjectCore}};

pub struct PlayerInputController{

}


impl PlayerInputController{
    pub fn new() -> PlayerInputController {
        return PlayerInputController {  };
    }
}


impl GameObjectController for PlayerInputController{
    fn update(&mut self, core: &mut GameObjectCore, input: &InputHandler) {
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
    }
}