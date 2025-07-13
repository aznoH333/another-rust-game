use crate::{engine::objects::{game_object::{GameObject, GameObjectBuilder}, spawning::object_summon::{ObjectSummonParameters, ObjectSummonRegistration}}, game::enums::drawing_layers::DrawingLayer};

fn shopkeeper_new(parameters: &ObjectSummonParameters) -> GameObject {
    return GameObjectBuilder::new(parameters.x, parameters.y, "shop_keeper_0001", DrawingLayer::WorldObjects.get_value()).build();
}

inventory::submit! {
    ObjectSummonRegistration::new("shopkeeper", shopkeeper_new)
}