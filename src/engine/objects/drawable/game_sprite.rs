use std::collections::HashMap;
use ggez::{graphics::Color};

use crate::engine::drawing::drawing_manager::DrawingManager;
use crate::engine::objects::game_box::GameBox;
use crate::engine::{objects::drawable::game_object_animation::GameObjectAnimation, world::world_constants::TILE_SIZE};


pub struct GameSprite{
    x: f32,
    y: f32,
    width: f32,
    height: f32,
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
            x: x, 
            y: y, 
            width: TILE_SIZE as f32, 
            height: TILE_SIZE as f32, 
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

    pub fn draw(&mut self, drawing_manager: &mut DrawingManager) {
        let mut sprite_name = &self.sprite_name;
        let x = self.left();
        let y = self.top();
        if self.use_animations {
            let animation = self.animations.get_mut(&self.current_animation).expect(format!("Animation not found {}", self.current_animation).as_str());

            sprite_name = animation.get_current_frame();
        }
        // drawing
        drawing_manager.draw_sprite(sprite_name, x + self.sprite_x_offset, y + self.sprite_y_offset, self.z_index, self.scale, self.flip_sprite, self.rotation, self.color);
    }

    pub fn update_animations(&mut self, delta: f32) {
        if self.use_animations {
            let animation = self.animations.get_mut(&self.current_animation).expect(format!("Animation not found {}", self.current_animation).as_str());
            animation.update_animation(delta);
        }
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
}

impl GameBox for GameSprite {
    /**
     * returns the "left" side of object -> x position
     */
    fn left(&self) -> f32 {
        return self.x - (self.width / 2.0);
    }

    /**
     * returns the "right" side of object -> x position
     */
    fn right(&self) -> f32 {
        return self.x + (self.width / 2.0);
    }

    /**
     * returns the "top" side of object -> y position
     */
    fn top(&self) -> f32 {
        return self.y - (self.height / 2.0);
    }

    /**
     * returns the "bottom" side of object -> y position
     */
    fn bottom(&self) -> f32 {
        return self.y + (self.height / 2.0);
    }

    fn get_x(&self) -> f32 {
        return self.x;
    } 

    fn get_y(&self) -> f32 {
        return self.y;
    }

    fn get_width(&self) -> f32 {
        return self.width;
    }

    fn get_height(&self) -> f32 {
        return self.height;
    }

    fn get_id(&self) -> u32 {
        return 0; // TODO : this shouldnt be here
    }
}
