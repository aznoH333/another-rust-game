
use crate::engine::objects::drawable::game_sprite::GameSprite;
use crate::{engine::{drawing::drawing_manager::DrawingManager, events::event_manager::{self, EventManager}, objects::{drawable::engine_animations::ANIMATION_HURT, drawable::game_object_animation::GameObjectAnimation, object_simplification::ObjectSimplification, object_weapon::ObjectWeapon}, types::vector::Vector, world::{world_constants::TILE_SIZE, world_manager::WorldManager}}, utils::{number_utils::NumberUtils, space_utils::SpaceUtils}};
use crate::engine::objects::game_box::GameBox;
use crate::engine::objects::drawable::engine_animations::{ANIMATION_IDLE, ANIMATION_WALK};
use crate::engine::utils::timer::Timer;
// TODO restructure engine directory to make sense
// TODO fix these garbage auto imports
pub struct GameObjectCore {
    

    pub sprite: GameSprite,
    

    // movement and physics
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub friction: f32,
    pub bounciness: f32,
    pub speed: f32,
    pub acceleration: f32,
    pub normalize_friction: bool,
    pub collide_with_terrain: bool,
    // number between -1 and 1 indicating the direction the object wants to accelerate in
    pub movement_x: f32,
    pub movement_y: f32,

    // combat logic
    pub faction: u32,
    pub damage: f32,
    pub health: f32,
    pub name: String,
    pub stun_timer: Timer,
    pub weapon: Option<ObjectWeapon>,

    // drawing stuff
    pub is_camera_target: bool,
    pub allow_auto_flipping: bool,

    // state controll
    pub wants_to_live: bool,
    collided_with_world: bool,
    delta: f32,
    pub id: u32,
    is_ready_to_draw: bool,
    pub look_for_target_with_name: Option<String>,
}


impl GameObjectCore {
    pub fn new(x: f32, y: f32, sprite_name: &str, z_index: i32) -> GameObjectCore {
        return GameObjectCore{
            sprite: GameSprite::new(x, y, sprite_name, z_index),
            x_velocity: 0.0,
            y_velocity: 0.0,
            friction: 0.1,
            bounciness: 0.0,
            is_camera_target: false,
            wants_to_live: true,
            collided_with_world: false,
            delta: 0.0,
            faction: 0,
            damage: 0.0,
            health: 1.0,
            id: 0,
            is_ready_to_draw: false,
            name: String::new(),
            look_for_target_with_name: None,
            speed: 0.75,
            movement_x: 0.0,
            movement_y: 0.0,
            acceleration: 0.25,
            allow_auto_flipping: true,
            stun_timer: Timer::new(0),
            normalize_friction: true,
            collide_with_terrain: true,
            weapon: None,
        }
    }


    pub fn draw(&mut self, drawing_manager: &mut DrawingManager) {
        if !self.is_ready_to_draw || !self.wants_to_live {
            return;
        }
        self.sprite.draw(drawing_manager);
        // weapon
        if self.weapon.is_some() {
            let weapon = self.weapon.as_mut().unwrap();
            weapon.draw(drawing_manager);
        }
    }

    pub fn update(&mut self, world: &WorldManager, delta: f32) {
        self.is_ready_to_draw = true;
        self.delta = delta;
        // movement
        if self.stun_timer.can_activate() {
            self.accelerate_in_direction(self.movement_x, self.movement_y);
        }
        self.movement_x = 0.0;
        self.movement_y = 0.0;

        self.collided_with_world = world.move_in_world(self, delta);

        self.update_animation_state();

        if self.normalize_friction {
            // friction
            let direction = SpaceUtils::direction_towards(0.0, 0.0, self.x_velocity, self.y_velocity);
            self.x_velocity = NumberUtils::gravitate_number(self.x_velocity, 0.0, self.friction * delta * direction.cos());
            self.y_velocity = NumberUtils::gravitate_number(self.y_velocity, 0.0, self.friction * delta * direction.sin());
        }else {
            // legacy friction
            self.x_velocity = NumberUtils::gravitate_number(self.x_velocity, 0.0, self.friction * delta);
            self.y_velocity = NumberUtils::gravitate_number(self.y_velocity, 0.0, self.friction * delta);
        }
        
        
        // weapon
        if self.weapon.is_some() {
            let rust_x = self.get_x(); // love rust
            let rust_y = self.get_y();
            
            let weapon = self.weapon.as_mut().unwrap().update(rust_x, rust_y);
        }


        // animation
        self.sprite.update_animations(delta);
    }

    fn update_animation_state(&mut self) {
        if !self.sprite.uses_animations(){
            return;
        }

        // TODO : death animation?
        if !self.stun_timer.can_activate() {
            self.play_animation(ANIMATION_HURT, false);
        }else if self.x_velocity.abs() > self.speed * self.acceleration || self.y_velocity.abs() > self.speed * self.acceleration {
            self.play_animation(ANIMATION_WALK, false);
        }else {
            self.play_animation(ANIMATION_IDLE, false);
        }

        // flip sprite
        if self.allow_auto_flipping && self.stun_timer.can_activate(){
            if self.x_velocity < -self.speed * self.acceleration {
                self.sprite.set_flip(true);
            }else if self.x_velocity > self.speed * self.acceleration {
                self.sprite.set_flip(false);
            }
        }
    }

    pub fn die(&mut self) {
        self.wants_to_live = false;
    }

    pub fn get_center_position(&self) -> Vector{
        return Vector{
            x: self.get_x(), 
            y: self.get_y()
        };
    }

    pub fn get_position(&self) -> Vector {
        return Vector{
            x: self.get_x(), 
            y: self.get_y()
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

    pub fn play_animation(&mut self, animation: i32, reset: bool) {
        self.sprite.play_animation(animation, reset);
    }

    pub fn set_target(&mut self, target: String) {
        self.look_for_target_with_name = Some(target);
    }

    pub fn get_simplification(&self) -> ObjectSimplification {
        return ObjectSimplification::new(self);
    }

    pub fn accelerate_in_direction(&mut self, x: f32, y: f32) {
        let direction = SpaceUtils::direction_towards(0.0, 0.0, x, y);
        let speed = SpaceUtils::get_vec_length(x, y);

        self.x_velocity = NumberUtils::gravitate_number(self.x_velocity, self.speed * direction.cos(), x * self.speed * self.delta * self.acceleration);
        self.y_velocity = NumberUtils::gravitate_number(self.y_velocity, self.speed * direction.sin(), y * self.speed * self.delta * self.acceleration);
    }

    pub fn set_weapon(&mut self, weapon: Option<ObjectWeapon>) {
        self.weapon = weapon;
    }

    pub fn attack(&mut self, event_manager: &mut EventManager) {
        if self.weapon.is_some() {
            self.weapon.as_mut().unwrap().fire(event_manager);
        }
    }

    pub fn set_weapon_direction(&mut self, direction: f32) {
        if self.weapon.is_some() {
            self.weapon.as_mut().unwrap().set_direction(direction);
        }
    }

}

impl GameBox for GameObjectCore {
    /**
     * returns the "left" side of object -> x position
     */
    fn left(&self) -> f32 {
        return self.get_x() - (self.get_width() / 2.0);
    }

    /**
     * returns the "right" side of object -> x position
     */
    fn right(&self) -> f32 {
        return self.get_x() + (self.get_width() / 2.0);
    }

    /**
     * returns the "top" side of object -> y position
     */
    fn top(&self) -> f32 {
        return self.get_y() - (self.get_height() / 2.0);
    }

    /**
     * returns the "bottom" side of object -> y position
     */
    fn bottom(&self) -> f32 {
        return self.get_y() + (self.get_height() / 2.0);
    }

    fn get_x(&self) -> f32 {
        return self.sprite.get_x();
    } 

    fn get_y(&self) -> f32 {
        return self.sprite.get_y();
    }

    fn get_width(&self) -> f32 {
        return self.sprite.get_width();
    }

    fn get_height(&self) -> f32 {
        return self.sprite.get_height();
    }

    fn get_id(&self) -> u32 {
        return self.id;
    }
}