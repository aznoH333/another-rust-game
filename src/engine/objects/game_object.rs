use std::collections::HashMap;

use crate::engine::{drawing::drawing_manager::DrawingManager, objects::{drawable::game_object_animation::GameObjectAnimation, object_simplification::ObjectSimplification, object_update::ObjectUpdate, object_weapon::ObjectWeapon}, types::vector::Vector, world::world_manager::WorldManager};

use super::{game_object_controller::GameObjectController, game_object_core::GameObjectCore};
use ggez::graphics::Color;
pub struct GameObject{
    core: GameObjectCore,
    controllers: HashMap<u8, Vec::<Box<dyn GameObjectController>>>,
}

impl GameObject{
    pub fn new(core: GameObjectCore, controllers: HashMap<u8, Vec::<Box<dyn GameObjectController>>>) -> GameObject{
        return GameObject{
            core,
            controllers,
        }
    }

    pub fn draw(&mut self, drawing_manager: &mut DrawingManager) {
        self.core.draw(drawing_manager);
    }

    pub fn update(&mut self, world: &WorldManager, delta: f32){
        self.core.update(world, delta);
    }

    pub fn activate_event(&mut self, update: &mut ObjectUpdate) {
        
        let controllers_to_update = self.controllers.get_mut(&update.event.event_type);

        if controllers_to_update.is_none() {
            return;
        }

        for controller in controllers_to_update.unwrap(){
            controller.update(&mut self.core, update);
        }
    }

    pub fn is_camera_target(&self) -> bool {
        return self.core.is_camera_target;
    }

    pub fn get_x(&self) -> f32 {
        return self.core.get_x();
    }

    pub fn get_y(&self) -> f32 {
        return self.core.get_y();
    }

    pub fn get_width(&self) -> f32 {
        return self.core.get_width();
    }

    pub fn get_height(&self) -> f32 {
        return self.core.get_height();
    }

    pub fn is_alive(&self) -> bool {
        return self.core.wants_to_live;
    }

    pub fn get_position(&self) -> Vector {
        return self.core.get_position();
    }

    pub fn get_center_position(&self) -> Vector {
        return self.core.get_center_position();
    }
    
    pub fn collided_with_world(&self) -> bool {
        return self.core.collided_with_world();
    }

    pub fn collides_with_object(&self, other: &GameObject) -> bool {
        return self.core.collides_with_box(&other.core.sprite.position);
    }

    pub fn equals(&self, other: &GameObject) -> bool {
        return self.core.id == other.core.id;
    }

    pub fn get_damage(&self) -> f32 {
        return self.core.damage;
    }

    pub fn get_faction(&self) -> u32 {
        return self.core.faction;
    }

    pub fn get_simplification(&self) -> ObjectSimplification {
        return self.core.get_simplification();
    }

    pub fn get_target(&self) -> &Option<String> {
        return &self.core.look_for_target_with_name;
    }


    /**
     * Note to self.
     * Can realy fuckup an object if interacted with improperly
     * Prefer game object methods instead of core.
     */
    pub fn get_core(&self) -> &GameObjectCore {
        return &self.core;
    }
}


pub struct GameObjectBuilder{
    core: GameObjectCore,
    controllers: HashMap<u8, Vec::<Box<dyn GameObjectController>>>,
}


impl GameObjectBuilder{
    pub fn new(x: f32, y: f32, sprite_name: &str, z_index: i32) -> GameObjectBuilder {
        return GameObjectBuilder { 
            core: GameObjectCore::new(x, y, sprite_name, z_index),
            controllers: HashMap::new(),
        };
    }

    pub fn set_dimensions(mut self, width: f32, height: f32) -> GameObjectBuilder {
        self.core.set_width(width);
        self.core.set_height(height);
        return self;
    }

    pub fn set_camera_target(mut self) -> GameObjectBuilder {
        self.core.is_camera_target = true;
        return self;
    }


    pub fn set_sprite_offset(mut self, x_offset: f32, y_offset: f32) -> GameObjectBuilder {
        self.core.set_sprite_x_offset(x_offset);
        self.core.set_sprite_y_offset(y_offset);
        return self;
    }


    pub fn add_controller(mut self, controller_type: u8, controller: Box::<dyn GameObjectController>) -> GameObjectBuilder{
        
        if !self.controllers.contains_key(&controller_type) {
            self.controllers.insert(controller_type, Vec::new());
        }
        self.controllers.get_mut(&controller_type).unwrap().push(controller);
        return self;
    }

    pub fn add_animation(mut self, animation_index: i32, animation: GameObjectAnimation) -> GameObjectBuilder {
        self.core.add_animation(animation_index, animation);

        return self
    }

    pub fn set_damage(mut self, damage: f32) -> GameObjectBuilder {
        self.core.damage = damage;
        return self;
    }

    pub fn set_faction(mut self, faction: u32) -> GameObjectBuilder {
        self.core.faction = faction;
        return self;
    }

    pub fn set_health(mut self, health: f32) -> GameObjectBuilder {
        self.core.health = health;
        return self;
    }

    pub fn set_name(mut self, name: &str) -> GameObjectBuilder {
        self.core.name = name.to_owned();
        return self;
    }

    pub fn set_target(mut self, target: &str) -> GameObjectBuilder {
        self.core.set_target(target.to_owned());
        return self;
    }
    
    pub fn set_speed(mut self, speed: f32) -> GameObjectBuilder {
        self.core.speed = speed;
        return self;
    }

    pub fn set_acceleration(mut self, acceleration: f32) -> GameObjectBuilder {
        self.core.acceleration = acceleration;
        return self;
    }

    pub fn set_friction(mut self, friction: f32) -> GameObjectBuilder {
        self.core.acceleration = friction;
        return self;
    }

    pub fn disable_auto_flipping(mut self) -> GameObjectBuilder {
        self.core.allow_auto_flipping = false;
        return self;
    }

    pub fn set_stun_length(mut self, length: u128) -> GameObjectBuilder {
        self.core.stun_timer.set_cooldown(length);
        return self;
    }

    pub fn set_bounciness(mut self, bounciness: f32) -> GameObjectBuilder{
        self.core.bounciness = bounciness;
        return self;
    }

    pub fn set_starting_velocity(mut self, x: f32, y: f32) -> GameObjectBuilder {
        self.core.x_velocity = x;
        self.core.y_velocity = y;
        return self;
    }

    pub fn set_color(mut self, color: Color) -> GameObjectBuilder {
        self.core.set_color(color);
        return self;
    }

    pub fn disable_friction_normalization(mut self) -> GameObjectBuilder {
        self.core.normalize_friction = false;
        return self;
    }

    pub fn disable_terrain_collisions(mut self) -> GameObjectBuilder {
        self.core.collide_with_terrain = false;
        return self;
    }

    pub fn set_weapon(mut self, weapon: Option<ObjectWeapon>) -> GameObjectBuilder {
        self.core.set_weapon(weapon);
        return self;
    }

    pub fn build(self) -> GameObject{
        return GameObject { core: self.core, controllers: self.controllers }
    }


    

}