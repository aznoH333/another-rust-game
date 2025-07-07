use crate::{engine::{objects::{game_object::{GameObject, GameObjectBuilder}, object_summon::{ObjectSummonParameters, ObjectSummonRegistration}}, types::controller_type::{CONTROLLER_TYPE_DESTROYED, CONTROLLER_TYPE_OBJECT_COLLIDE}}, game::{entities::{controllers::{spawn_giblets_on_death::SpawnGibletsOnDeathController, take_damage_on_hostile_collision::TakeDamageOnHostileCollisionController}, factions::FACTION_NEUTRAL, objects::effects::giblet_type::GIBLET_SPLINTER}, enums::drawing_layers::DrawingLayer}, utils::{number_utils::NumberUtils, textures::TextureUtils}};

use ggez::graphics::Color;

fn treasure_new(parameters: &ObjectSummonParameters) -> GameObject {
    return GameObjectBuilder::new(parameters.x, parameters.y, &TextureUtils::get_texture_with_index("tiles", NumberUtils::random_integer_from_array(&[18, 20, 22])), DrawingLayer::WorldObjects.get_value())
    .set_faction(FACTION_NEUTRAL)
    .set_health(10.0)
    .add_controller(CONTROLLER_TYPE_OBJECT_COLLIDE, Box::new(TakeDamageOnHostileCollisionController::new(200, 0.0)))
    .add_controller(CONTROLLER_TYPE_DESTROYED, Box::new(SpawnGibletsOnDeathController::new(GIBLET_SPLINTER, 2, 5, vec!(Color::new(0.902, 0.784, 0.431, 1.0)))))
    .build();
}

inventory::submit! {
    ObjectSummonRegistration::new("treasure", treasure_new)
}