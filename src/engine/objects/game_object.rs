use crate::engine::{drawing::drawing_manager::DrawingManager, input::input::InputHandler, world::world_manager::WorldManager};

use super::{game_object_controller::GameObjectController, game_object_core::GameObjectCore};

pub struct GameObject{
    core: GameObjectCore,
    controller: Box<dyn GameObjectController>,
}

impl GameObject{
    pub fn new(core: GameObjectCore, controller: Box<dyn GameObjectController>) -> GameObject{
        return GameObject{
            core,
            controller
        }
    }

    pub fn update(&mut self, drawing_manager: &mut DrawingManager, input: &InputHandler, world: &WorldManager){
        self.core.update(drawing_manager, world);
        self.controller.update(&mut self.core, input);
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