use crate::{engine::{drawing::drawing_manager::DrawingManager, world::{world_constants::TILE_SIZE, world_manager::WorldManager}}, utils::number_utils::NumberUtils};

pub struct GameObjectCore {
    
    // positional stuff
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub sprite_x_offset: f32,
    pub sprite_y_offset: f32,

    // movement and physics
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub friction: f32,
    pub bouncyness: f32,
    
    // drawing stuff
    pub sprite_name: String,
    pub z_index: i32,
    pub scale: f32,
    pub is_camera_target: bool,
}


impl GameObjectCore {
    pub fn new(x: f32, y: f32, sprite_name: &str, z_index: i32) -> GameObjectCore {
        return GameObjectCore{
            x,
            y,
            width: TILE_SIZE as f32,
            height: TILE_SIZE as f32,
            sprite_x_offset: 0.0, 
            sprite_y_offset: 0.0,
            x_velocity: 0.0,
            y_velocity: 0.0,
            friction: 0.1,
            bouncyness: 0.0,
            sprite_name: sprite_name.to_owned(),
            z_index,
            scale: 1.0,
            is_camera_target: false,
        }
    }


    pub fn update(&mut self, drawing_manager: &mut DrawingManager, world: &WorldManager) {
        // movement
        world.move_in_world(self);

        // friction
        self.x_velocity = NumberUtils::gravitate_number(self.x_velocity, 0.0, self.friction);
        self.y_velocity = NumberUtils::gravitate_number(self.y_velocity, 0.0, self.friction);

        // drawing
        drawing_manager.draw_sprite(&self.sprite_name, self.x + self.sprite_x_offset, self.y + self.sprite_y_offset, self.z_index, self.scale);
    }
}