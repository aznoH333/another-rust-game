use crate::{engine::{objects::game_object::{GameObject, GameObjectBuilder}, types::controller_type::CONTROLLER_TYPE_UPDATE}, game::{entities::controllers::player_input_controller::PlayerInputController, enums::drawing_layers::DrawingLayer}};

pub struct Player{

}



impl Player{
    pub fn new(x: f32, y: f32) -> GameObject{
        return GameObjectBuilder::new(x, y, "player_0001", DrawingLayer::Player.get_value())
        // set core values
        .set_dimensions(10.0, 10.0)
        .set_sprite_offset(-3.0, -6.0)
        .set_camera_target()
        // controllers
        .add_controller(CONTROLLER_TYPE_UPDATE,Box::new(PlayerInputController::new()))
        .build();
    }
}
