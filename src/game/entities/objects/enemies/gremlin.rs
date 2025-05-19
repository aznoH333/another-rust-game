use crate::{engine::objects::game_object::{GameObject, GameObjectBuilder}, game::enums::drawing_layers::DrawingLayer};

pub struct Gremlin {

}

impl Gremlin {
    pub fn new(x: f32,y: f32) -> GameObject {
        return 
            GameObjectBuilder::new(x, y, "gremlin_0001", DrawingLayer::GameObjects.get_value())
            .set_dimensions(10.0, 10.0)
            .set_sprite_offset(-3.0, -6.0)

            .build()
            ;
    }
}