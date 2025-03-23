use crate::{engine::drawing::drawing_manager::DrawingManager, game::enums::drawing_layers::DrawingLayer};

pub struct WorldTile {
    solid: bool,
    texture: String,
    x: i32,
    y: i32,
}


impl WorldTile{
    pub fn new(solid: bool, texture: &str, x: i32, y: i32) -> WorldTile {
        return WorldTile{
            solid: solid,
            texture: texture.to_owned(),
            x: x,
            y: y,
        };
    }

    pub fn is_solid(&self) -> bool {
        return self.solid;
    }

    pub fn get_texture(&self) -> &String {
        return &self.texture;
    }

    pub fn set_texture(&mut self, texture: &str){
        self.texture = texture.to_owned();
    }

    pub fn set_solid(&mut self, solid: bool){
        self.solid = solid;
    }

    pub fn draw(&self, drawing: &mut DrawingManager){
        drawing.draw_sprite(&self.texture, self.x as f32, self.y as f32, DrawingLayer::WORLD as i32, 1.0);
    } // TODO : world generator
}