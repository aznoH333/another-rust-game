use crate::{engine::objects::game_object::{GameObject, GameObjectBuilder}, game::enums::drawing_layers::DrawingLayer, utils::{number_utils::NumberUtils, textures::TextureUtils}};

pub struct Treasure{

}


impl Treasure {
    pub fn new(x: f32, y: f32) -> GameObject {
        return GameObjectBuilder::new(x, y, &TextureUtils::get_texture_with_index("tiles", NumberUtils::random_integer_from_array(&[18, 20, 22])), DrawingLayer::WorldObjects.get_value()).build();
    }
}