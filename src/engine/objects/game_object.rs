use crate::engine::{drawing::drawing_manager::DrawingManager, input::input::InputHandler};

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

    pub fn update(&mut self, drawing_manager: &mut DrawingManager, input: &InputHandler){
        self.core.update(drawing_manager);
        self.controller.update(&mut self.core, input);
    }
}