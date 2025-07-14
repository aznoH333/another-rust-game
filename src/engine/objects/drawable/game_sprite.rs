use std::collections::HashMap;
use ggez::{graphics::Color};

use crate::engine::drawing::drawing_manager::DrawingManager;
use crate::engine::objects::game_box::GameBox;
use crate::engine::{objects::drawable::game_object_animation::GameObjectAnimation, world::world_constants::TILE_SIZE};


pub struct GameSprite{
    pub position: GameBox,
    sprite_x_offset: f32,
    sprite_y_offset: f32,

    sprite_name: String,
    z_index: i32,
    scale: f32,

    flip_sprite: bool,
    rotation: f32,
    color: Color,

    current_animation: i32,
    animations: HashMap<i32, GameObjectAnimation>,
    use_animations: bool,

}


impl GameSprite {
    pub fn new(x: f32, y: f32, sprite: &str, z_index: i32) -> GameSprite {
        return GameSprite { 
            position: GameBox::new(x, y, TILE_SIZE as f32, TILE_SIZE as f32),
            sprite_x_offset: 0.0, 
            sprite_y_offset: 0.0, 
            sprite_name: sprite.to_owned(), 
            z_index: z_index, 
            scale: 1.0, 
            flip_sprite: false, 
            rotation: 0.0, 
            color: Color::new(1.0, 1.0, 1.0, 1.0 ),
            current_animation: 0, 
            animations: HashMap::new(),
            use_animations: false,
        }
    }

    pub fn draw(&self, drawing_manager: &mut DrawingManager) {
        let mut sprite_name = &self.sprite_name;
        let x = self.position.left();
        let y = self.position.top();
        if self.use_animations {
            let animation = self.animations.get(&self.current_animation).expect(format!("Animation not found {}", self.current_animation).as_str());

            sprite_name = animation.get_current_frame();
        }
        // drawing
        drawing_manager.draw_sprite(sprite_name, x + self.sprite_x_offset, y + self.sprite_y_offset, self.z_index, self.scale, self.flip_sprite, self.rotation, self.color);
    }


    pub fn play_animation(&mut self, animation: i32, reset: bool) {
        self.current_animation = animation;
        if reset {
            self.animations.get_mut(&animation).unwrap().reset_animation();
        }
    }

    pub fn add_animation(&mut self, index: i32, animation: GameObjectAnimation) {
        self.animations.insert(index, animation);
        self.use_animations = true;
    }

    pub fn uses_animations(&self) -> bool {
        return self.use_animations;
    }

    pub fn set_flip(&mut self, flip: bool) {
        self.flip_sprite = flip;
    }

    pub fn get_y(&self) -> f32 {
        return self.position.y;
    }

    pub fn get_width(&self) -> f32 {
        return self.position.width;
    }

    pub fn get_height(&self) -> f32 {
        return self.position.height;
    }

    pub fn set_x(&mut self, value: f32) {
        self.position.x = value;
    }

    pub fn set_y(&mut self, value: f32) {
        self.position.y = value;
    }

    pub fn set_width(&mut self, value: f32) {
        self.position.width = value;
    }

    pub fn set_height(&mut self, value: f32) {
        self.position.height = value;
    }

    pub fn set_sprite_x_offset(&mut self, offset: f32) {
        self.sprite_x_offset = offset;
    }

    pub fn set_sprite_y_offset(&mut self, offset: f32) {
        self.sprite_y_offset = offset;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn get_color_mut(&mut self) -> &mut Color {
        return &mut self.color;
    }

    pub fn get_rotation(&self) -> f32 {
        return self.rotation;
    }
}
