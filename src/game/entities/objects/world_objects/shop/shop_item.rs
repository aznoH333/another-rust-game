use crate::{engine::objects::game_object::{GameObject, GameObjectBuilder}, game::enums::drawing_layers::DrawingLayer};

pub struct ShopItem{

}


impl ShopItem {
    pub fn new(x: f32, y: f32) -> GameObject{
        return GameObjectBuilder::new(x, y, "bow_0002", DrawingLayer::WorldObjects.get_value()).build();
    }
}