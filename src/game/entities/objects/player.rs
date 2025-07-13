use crate::{engine::{objects::{engine_animations::{ANIMATION_IDLE, ANIMATION_WALK}, game_object::{GameObject, GameObjectBuilder}, game_object_animation::GameObjectAnimation, object_weapon::ObjectWeapon}, types::controller_type::CONTROLLER_TYPE_UPDATE}, game::{entities::controllers::player_input_controller::PlayerInputController, enums::drawing_layers::DrawingLayer}};
use crate::game::entities::factions::FACTION_PLAYER;
use crate::engine::objects::spawning::object_summon::ObjectSummonRegistration;
use crate::engine::objects::spawning::object_summon::ObjectSummonParameters;

fn player_new(parameters: &ObjectSummonParameters) -> GameObject{

    return GameObjectBuilder::new(parameters.x, parameters.y, "player_0001", DrawingLayer::Player.get_value())
    // set core values
    .set_dimensions(10.0, 10.0)
    .set_sprite_offset(-3.0, -6.0)
    .set_camera_target()
    // stats
    .set_speed(1.2)
    .disable_friction_normalization()
    // combat
    .set_faction(FACTION_PLAYER)
    .set_health(100.0)
    .set_name("player")
    .set_weapon(
        Some(ObjectWeapon::new(250, "bow_0002"))
    )
    // animations
    // idle animation
    .add_animation(ANIMATION_IDLE, GameObjectAnimation::new(9.0).add_frame("player_0001"))
    // walk animation
    .add_animation(ANIMATION_WALK, GameObjectAnimation::new(9.0).add_frame("player_0002").add_frame("player_0003"))
    // controllers
    .add_controller(CONTROLLER_TYPE_UPDATE,Box::new(PlayerInputController::new()))
    .build();
}
inventory::submit! {
    ObjectSummonRegistration::new("player", player_new)
}