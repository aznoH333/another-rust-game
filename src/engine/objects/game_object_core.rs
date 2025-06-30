use crate::{engine::{drawing::drawing_manager::DrawingManager, objects::game_object_animation::GameObjectAnimation, types::vector::Vector, world::{world_constants::TILE_SIZE, world_manager::WorldManager}}, utils::number_utils::NumberUtils};

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
    
    // combat logic
    pub faction: u32,
    pub damage: f32,
    pub health: f32,

    // drawing stuff
    pub sprite_name: String,
    pub z_index: i32,
    pub scale: f32,
    pub is_camera_target: bool,
    pub flip_sprite: bool,

    pub current_animation: usize,
    pub animations: Vec<GameObjectAnimation>,
    pub use_animations: bool,
    // state controll
    pub wants_to_live: bool,
    collided_with_world: bool,
    delta: f32,
    pub id: u32,

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
            current_animation: 0,
            animations: Vec::new(),
            use_animations: false,
            is_camera_target: false,
            flip_sprite: false,
            wants_to_live: true,
            collided_with_world: false,
            delta: 0.0,
            faction: 0,
            damage: 0.0,
            health: 1.0,
            id: 0,
        }
    }


    pub fn draw(&mut self, drawing_manager: &mut DrawingManager) {
        let mut sprite_name = &self.sprite_name;

        if self.use_animations {
            let mut animation = self.animations.get_mut(self.current_animation).expect(format!("Animation not found {}", self.current_animation).as_str());

            sprite_name = animation.get_current_frame();
        }
        // drawing
        drawing_manager.draw_sprite(sprite_name, self.x + self.sprite_x_offset, self.y + self.sprite_y_offset, self.z_index, self.scale, self.flip_sprite);
    }

    pub fn update(&mut self, world: &WorldManager, delta: f32) {
        self.delta = delta;
        // movement
        self.collided_with_world = world.move_in_world(self, delta);

        // friction
        self.x_velocity = NumberUtils::gravitate_number(self.x_velocity, 0.0, self.friction * delta);
        self.y_velocity = NumberUtils::gravitate_number(self.y_velocity, 0.0, self.friction * delta);

        // animation
        if self.use_animations {
            let mut animation = self.animations.get_mut(self.current_animation).expect(format!("Animation not found {}", self.current_animation).as_str());
            animation.update_animation(delta);
        }
    }

    pub fn die(&mut self) {
        self.wants_to_live = false;
    }

    pub fn get_center_position(&self) -> Vector{
        return Vector{
            x: self.x + (self.width / 2.0), 
            y: self.y + (self.height / 2.0)
        };
    }

    pub fn get_position(&self) -> Vector {
        return Vector{
            x: self.x, 
            y: self.y
        };
    }

    pub fn collided_with_world(&self) -> bool {
        return self.collided_with_world;
    }

    pub fn get_x_velocity(&self) -> f32 {
        return self.x_velocity * self.delta;
    }

    pub fn get_y_velocity(&self) -> f32 {
        return self.y_velocity * self.delta;
    }

    pub fn play_animation(&mut self, animation: usize, reset: bool) {
        self.current_animation = animation;
        if reset {
            self.animations.get_mut(animation).unwrap().reset_animation();
        }
    }
}