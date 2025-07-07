use crate::{engine::{objects::{game_object::{GameObject, GameObjectBuilder}, object_summon::{ObjectSummonParameters, ObjectSummonRegistration}}, types::controller_type::{CONTROLLER_TYPE_DESTROYED, CONTROLLER_TYPE_OBJECT_COLLIDE, CONTROLLER_TYPE_UPDATE, CONTROLLER_TYPE_WORLD_COLLIDE}}, game::{entities::{controllers::{destroy_on_event::DestroyOnEvent, destroy_on_hostile_collision::DestroyOnHostileCollision, projectile_controller::ProjectileController, spawn_giblets_on_death::SpawnGibletsOnDeathController}, objects::effects::giblet_type::GIBLET_SPLINTER}, enums::drawing_layers::DrawingLayer}};
use ggez::graphics::Color;

fn bullet_new(parameters: &ObjectSummonParameters) -> GameObject {
    return GameObjectBuilder::new(parameters.x, parameters.y, parameters.sprite, DrawingLayer::WorldObjects.get_value())
    .set_dimensions(2.0, 2.0)
    .set_sprite_offset(-7.0, -7.0)
    .set_faction(parameters.faction)
    .set_damage(parameters.damage)
    .set_name("projectile")
    .disable_auto_flipping()
    .add_controller(CONTROLLER_TYPE_UPDATE,Box::new(ProjectileController::new(parameters.direction, parameters.speed)))
    .add_controller(CONTROLLER_TYPE_WORLD_COLLIDE, Box::new(DestroyOnEvent::new()))
    .add_controller(CONTROLLER_TYPE_OBJECT_COLLIDE, Box::new(DestroyOnHostileCollision::new()))
    .add_controller(CONTROLLER_TYPE_DESTROYED, Box::new(SpawnGibletsOnDeathController::new(GIBLET_SPLINTER, 1, 3, vec!(Color::new(0.843, 0.451, 0.333, 1.0), Color::new(0.863, 0.961, 1.0, 1.0)))))
    .build();
}

inventory::submit! {
    ObjectSummonRegistration::new("projectile", bullet_new)
}