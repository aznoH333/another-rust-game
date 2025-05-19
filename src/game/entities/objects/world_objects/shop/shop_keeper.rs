use crate::{engine::objects::game_object::{GameObject, GameObjectBuilder}, game::enums::drawing_layers::DrawingLayer};

pub struct ShopKeeper{

}

impl ShopKeeper {
    pub fn new(x: f32, y: f32) -> GameObject {
        return GameObjectBuilder::new(x, y, "shop_keeper_0001", DrawingLayer::WorldObjects.get_value()).build();
    }
}