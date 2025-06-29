use crate::{engine::drawing::drawing_manager::DrawingManager, game::enums::drawing_layers::DrawingLayer};

use super::world_constants::TILE_SIZE;

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
        drawing.draw_sprite(&self.texture, self.x as f32, self.y as f32, DrawingLayer::World as i32, 1.0, false);
    } // TODO : world generator

    pub fn get_left(&self) -> f32 {
        return self.x as f32;
    }

    pub fn get_right(&self) -> f32 {
        return (self.x + TILE_SIZE) as f32;
    }

    pub fn get_top(&self) -> f32 {
        return self.y as f32;
    }

    pub fn get_bottom(&self) -> f32 {
        return (self.y + TILE_SIZE) as f32;
    }
}