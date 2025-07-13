use crate::{engine::objects::{game_object::{GameObject, GameObjectBuilder}, spawning::object_summon::{ObjectSummonParameters, ObjectSummonRegistration}}, game::enums::drawing_layers::DrawingLayer};

fn shop_item_new(parameters: &ObjectSummonParameters) -> GameObject{
    return GameObjectBuilder::new(parameters.x, parameters.y, "bow_0002", DrawingLayer::WorldObjects.get_value()).build();
}


inventory::submit! {
    ObjectSummonRegistration::new("shop_item", shop_item_new)
}
