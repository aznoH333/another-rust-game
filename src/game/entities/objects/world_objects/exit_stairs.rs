use crate::{engine::objects::{game_object::{GameObject, GameObjectBuilder}, spawning::object_summon::{ObjectSummonParameters, ObjectSummonRegistration}}, game::enums::drawing_layers::DrawingLayer};


fn exit_new(parameters: &ObjectSummonParameters) -> GameObject {
    return GameObjectBuilder::new(parameters.x, parameters.y, "tiles_0028", DrawingLayer::WorldObjects.get_value()).build();
}


inventory::submit! {
    ObjectSummonRegistration::new("exit", exit_new)
}

