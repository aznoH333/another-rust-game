use crate::engine::{drawing::drawing_manager::DrawingManager, events::event_manager::{self, EventManager}, input::input::InputHandler, world::world_manager::WorldManager};

use super::{game_object_controller::GameObjectController, game_object_core::GameObjectCore};

pub struct GameObject{
    core: GameObjectCore,
    controllers: Vec::<Box<dyn GameObjectController>>,
}

impl GameObject{
    pub fn new(core: GameObjectCore, controllers: Vec::<Box<dyn GameObjectController>>) -> GameObject{
        return GameObject{
            core,
            controllers,
        }
    }

    pub fn update(&mut self, drawing_manager: &mut DrawingManager, input: &InputHandler, world: &WorldManager, event_manager: &mut EventManager){
        self.core.update(drawing_manager, world);
        for controller in &mut self.controllers {
            controller.update(&mut self.core, input, event_manager);
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
}


pub struct GameObjectBuilder{
    core: GameObjectCore,
    controllers: Vec::<Box<dyn GameObjectController>>,
}


impl GameObjectBuilder{
    pub fn new(x: f32, y: f32, sprite_name: &str, z_index: i32) -> GameObjectBuilder {
        return GameObjectBuilder { 
            core: GameObjectCore::new(x, y, sprite_name, z_index),
            controllers: Vec::new(),
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
        self.core.x -= x_offset;
        self.core.y -= y_offset;
        return self;
    }


    pub fn add_controller(mut self, controller: Box::<dyn GameObjectController>) -> GameObjectBuilder{
        self.controllers.push(controller);
        return self;
    }


    pub fn build(self) -> GameObject{
        return GameObject { core: self.core, controllers: self.controllers }
    }
}