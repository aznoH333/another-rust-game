use crate::engine::drawing::drawing_manager::DrawingManager;

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
    pub gravity: f32,
    
    // drawing stuff
    pub sprite_name: String,

    // logic stuff

}


impl GameObjectCore {
    pub fn new(x: f32, y: f32, sprite_name: &str) -> GameObjectCore {
        return GameObjectCore{
            x: x,
            y: y,
            width: 16.0,
            height: 16.0,
            x_velocity: 0.0,
            y_velocity: 0.0,
            gravity: 1.0,
            sprite_name: sprite_name.to_owned(),
        }
    }


    pub fn update(&mut self, drawing_manager: &mut DrawingManager) {
        // TODO : movement and stuff

        // drawing
        drawing_manager.draw_sprite(&self.sprite_name, self.x, self.y, 0, 1.0);
    }
}