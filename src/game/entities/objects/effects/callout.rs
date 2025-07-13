use crate::{engine::{objects::{game_object::{GameObject, GameObjectBuilder}, object_summon::{ObjectSummonParameters, ObjectSummonRegistration}}, types::controller_type::CONTROLLER_TYPE_UPDATE}, game::{entities::controllers::{fade_in_and_out::FadeInAndOut}, enums::drawing_layers::DrawingLayer}};

fn callout_new(parameters: &ObjectSummonParameters) -> GameObject {
    return 
    GameObjectBuilder::new(parameters.x, parameters.y, parameters.sprite, DrawingLayer::GameObjects.get_value())
    .add_controller(CONTROLLER_TYPE_UPDATE, Box::new(FadeInAndOut::new(1000, 100, 200)))
    .set_starting_velocity(0.0, -2.5)
    .set_friction(0.001)
    .disable_terrain_collisions()
    .build();
}

inventory::submit! {
    ObjectSummonRegistration::new("callout", callout_new)
}