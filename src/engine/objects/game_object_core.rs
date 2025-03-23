use crate::{engine::drawing::drawing_manager::DrawingManager, utils::number_utils::gravitate_number};

use super::game_object_controller::GameObjectController;

pub struct GameObjectCore {
    
    // positional stuff
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    // movement and physics
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub friction: f32,
    
    // drawing stuff
    pub sprite_name: String,
    pub z_index: i32,
    pub scale: f32,
}


impl GameObjectCore {
    pub fn new(x: f32, y: f32, sprite_name: &str, z_index: i32) -> GameObjectCore {
        return GameObjectCore{
            x: x,
            y: y,
            width: 16.0,
            height: 16.0,
            x_velocity: 0.0,
            y_velocity: 0.0,
            friction: 0.1,
            sprite_name: sprite_name.to_owned(),
            z_index: z_index,
            scale: 1.0
        }
    }


    pub fn update(&mut self, drawing_manager: &mut DrawingManager) {
        // movement
        self.x += self.x_velocity;
        self.y += self.y_velocity;

        // friction
        self.x_velocity = gravitate_number(self.x_velocity, 0.0, self.friction);
        self.y_velocity = gravitate_number(self.y_velocity, 0.0, self.friction);

        // drawing
        drawing_manager.draw_sprite(&self.sprite_name, self.x, self.y, self.z_index, self.scale);
    }
}