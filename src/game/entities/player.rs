use crate::{engine::objects::{game_object::GameObject, game_object_controller::GameObjectController, game_object_core::GameObjectCore}, game::enums::drawing_layers::DrawingLayer};
use crate::InputHandler;

pub struct Player{

}



impl Player{
    pub fn new(x: f32, y: f32) -> GameObject{


        let controller = Player{

        };
        let mut core = GameObjectCore::new(x, y, "player_0001.png", DrawingLayer::PLAYER as i32);
        core.is_camera_target = true;

        return GameObject::new(
            core,
            Box::new(controller)
        );

    }
}


impl GameObjectController for Player{
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