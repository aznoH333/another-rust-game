use crate::{engine::{objects::{engine_animations::{ANIMATION_HURT, ANIMATION_IDLE, ANIMATION_WALK}, game_object::{GameObject, GameObjectBuilder}, game_object_animation::GameObjectAnimation, spawning::object_summon::{ObjectSummonParameters, ObjectSummonRegistration}}, types::controller_type::{CONTROLLER_TYPE_DESTROYED, CONTROLLER_TYPE_OBJECT_COLLIDE, CONTROLLER_TYPE_UPDATE}}, game::{entities::{controllers::{ai::fighter_controller::FighterController, take_damage_on_hostile_collision::TakeDamageOnHostileCollisionController}, objects::effects::giblet_type::GIBLET_BLOB}, enums::drawing_layers::DrawingLayer}};
use crate::game::entities::factions::FACTION_ENEMY;
use ggez::graphics::Color;
use crate::game::entities::controllers::spawn_giblets_on_death::SpawnGibletsOnDeathController;

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
        .add_animation(ANIMATION_HURT, 
            GameObjectAnimation::new(1.0)
        .add_frame("gremlin_0004"))
            
        // controllers
        .add_controller(CONTROLLER_TYPE_OBJECT_COLLIDE, Box::new(TakeDamageOnHostileCollisionController::new(200, 2.2)))
        .add_controller(CONTROLLER_TYPE_UPDATE, Box::new(FighterController::new("player")))
        .add_controller(CONTROLLER_TYPE_DESTROYED, Box::new(SpawnGibletsOnDeathController::new(GIBLET_BLOB, 5, 8, vec!(Color::new(0.893, 0.083, 0.083, 1.0)))))

        .build()
        ;
}

inventory::submit! {
    ObjectSummonRegistration::new("gremlin", gremlin_new)
}
