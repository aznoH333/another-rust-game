use crate::{engine::{objects::{engine_animations::{ANIMATION_IDLE, ANIMATION_WALK}, game_object::{GameObject, GameObjectBuilder}, game_object_animation::GameObjectAnimation}, types::controller_type::CONTROLLER_TYPE_UPDATE}, game::{entities::controllers::player_input_controller::PlayerInputController, enums::drawing_layers::DrawingLayer}};
use crate::game::entities::factions::FACTION_PLAYER;
pub struct Player{

}



impl Player{
    pub fn new(x: f32, y: f32) -> GameObject{
        return GameObjectBuilder::new(x, y, "player_0001", DrawingLayer::Player.get_value())
        // set core values
        .set_dimensions(10.0, 10.0)
        .set_sprite_offset(-3.0, -6.0)
        .set_camera_target()

        // stats
        .set_speed(1.2)

        // combat
        .set_faction(FACTION_PLAYER)
        .set_health(100.0)
        .set_name("player")

        // animations
        // idle animation
        .add_animation(ANIMATION_IDLE, GameObjectAnimation::new(9.0).add_frame("player_0001"))
        // walk animation
        .add_animation(ANIMATION_WALK, GameObjectAnimation::new(9.0).add_frame("player_0002").add_frame("player_0003"))

        // controllers
        .add_controller(CONTROLLER_TYPE_UPDATE,Box::new(PlayerInputController::new()))
        .build();
    }
}
