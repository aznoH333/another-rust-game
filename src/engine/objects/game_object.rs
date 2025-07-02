use std::{collections::HashMap, iter::Map};

use crate::{engine::{drawing::drawing_manager::DrawingManager, events::event_manager::EventManager, input::input::InputHandler, objects::game_object_animation::GameObjectAnimation, types::{object_event::ObjectEvent, vector::Vector}, world::{world_constants::TILE_SIZE, world_manager::WorldManager}}, utils::space_utils::SpaceUtils};

use super::{game_object_controller::GameObjectController, game_object_core::GameObjectCore};

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

    pub fn activate_event(&mut self, event: &ObjectEvent, input: &InputHandler, event_manager: &mut EventManager) {
        
        let controllers_to_update = self.controllers.get_mut(&event.event_type);

        if controllers_to_update.is_none() {
            return;
        }

        for controller in controllers_to_update.unwrap(){
            controller.update(&mut self.core, event, input, event_manager);
        }
    }

    pub fn is_camera_target(&self) -> bool {
        return self.core.is_camera_target;
    }

    pub fn get_x(&self) -> f32 {
        return self.core.x;
    }

    pub fn get_y(&self) -> f32 {
        return self.core.y;
    }

    pub fn get_width(&self) -> f32 {
        return self.core.width;
    }

    pub fn get_height(&self) -> f32 {
        return self.core.height;
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
        return SpaceUtils::squares_collide_f32(
            self.core.left(), 
            self.core.top(), 
            self.core.width, 
            self.core.height, 
            other.core.left(), 
            other.core.top(), 
            other.core.width, 
            other.core.height);
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
        self.core.width = width;
        self.core.height = height;
        return self;
    }

    pub fn set_camera_target(mut self) -> GameObjectBuilder {
        self.core.is_camera_target = true;
        return self;
    }


    pub fn set_sprite_offset(mut self, x_offset: f32, y_offset: f32) -> GameObjectBuilder {
        self.core.sprite_x_offset = x_offset;
        self.core.sprite_y_offset = y_offset;
        return self;
    }


    pub fn add_controller(mut self, controller_type: u8, controller: Box::<dyn GameObjectController>) -> GameObjectBuilder{
        
        if !self.controllers.contains_key(&controller_type) {
            self.controllers.insert(controller_type, Vec::new());
        }
        self.controllers.get_mut(&controller_type).unwrap().push(controller);
        return self;
    }

    pub fn add_animation(mut self, animation: GameObjectAnimation) -> GameObjectBuilder {
        self.core.animations.push(animation);
        self.core.use_animations = true;

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

    pub fn build(mut self) -> GameObject{
        return GameObject { core: self.core, controllers: self.controllers }
    }

}