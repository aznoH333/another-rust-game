use crate::{engine::{objects::{engine_animations::{ANIMATION_IDLE, ANIMATION_WALK}, game_object::{GameObject, GameObjectBuilder}, game_object_animation::GameObjectAnimation, object_summon::{ObjectSummonParameters, ObjectSummonRegistration}}, types::controller_type::{CONTROLLER_TYPE_OBJECT_COLLIDE, CONTROLLER_TYPE_UPDATE}}, game::{entities::{controllers::{fighter_controller::FighterController, take_damage_on_hostile_collision::TakeDamageOnHostileCollisionController}, objects::enemies::gremlin}, enums::drawing_layers::DrawingLayer}};
use crate::game::entities::factions::FACTION_ENEMY;

fn gremlin_new(parameters: &ObjectSummonParameters) -> GameObject {
    return 
        GameObjectBuilder::new(parameters.x, parameters.y, "gremlin_0001", DrawingLayer::GameObjects.get_value())
        .set_dimensions(10.0, 10.0)
        .set_sprite_offset(-3.0, -6.0)
        // stats
        .set_speed(0.65)
        // combat
        .set_faction(FACTION_ENEMY)
        .set_health(25.0)
        .set_name("gremlin")
        .set_target("player")
        .set_stun_length(500)
        // animations
        .add_animation(ANIMATION_IDLE, 
            GameObjectAnimation::new(10.0)
            .add_frame("gremlin_0001"))
        .add_animation(ANIMATION_WALK, 
            GameObjectAnimation::new(10.0)
            .add_frame("gremlin_0002")
            .add_frame("gremlin_0003")
        )
            
        // controllers
        .add_controller(CONTROLLER_TYPE_OBJECT_COLLIDE, Box::new(TakeDamageOnHostileCollisionController::new(200, 2.2)))
        .add_controller(CONTROLLER_TYPE_UPDATE, Box::new(FighterController::new("player")))
        .build()
        ;
}

inventory::submit! {
    ObjectSummonRegistration::new("gremlin", gremlin_new)
}
