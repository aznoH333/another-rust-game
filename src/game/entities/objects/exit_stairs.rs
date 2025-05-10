use crate::{engine::objects::game_object::{GameObject, GameObjectBuilder}, game::enums::drawing_layers::DrawingLayer};

pub struct ExitStairs{

}

impl ExitStairs{
    pub fn new(x: f32, y: f32) -> GameObject {
        return GameObjectBuilder::new(x, y, "tiles_0028", DrawingLayer::WorldObjects.get_value()).build();
    }
}