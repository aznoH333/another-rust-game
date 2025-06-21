use crate::engine::{drawing::drawing_manager::DrawingManager, events::event_manager::{self, EventManager}, input::input::InputHandler, world::world_manager::WorldManager};

use super::{game_object::GameObject, game_object_controller::GameObjectController};

pub struct GameObjectManager{
    game_objects: Vec<GameObject>,
}


impl GameObjectManager{
    pub fn new() -> GameObjectManager{
        return GameObjectManager{
            game_objects: Vec::new()
        }
    }


    pub fn update(&mut self, drawing_manager: &mut DrawingManager, input: &InputHandler, world: &WorldManager, event_manager: &mut EventManager) {

        self.update_objects(drawing_manager, input, world, event_manager);
        self.cull_dead_objects();
        self.update_camera(drawing_manager);

    }

    fn update_objects(&mut self, drawing_manager: &mut DrawingManager, input: &InputHandler, world: &WorldManager, event_manager: &mut EventManager) {
        for object in &mut self.game_objects{
            object.update(drawing_manager, input, world, event_manager);
        }
    }

    fn update_camera(&mut self, drawing_manager: &mut DrawingManager) {
        for object in &self.game_objects {
            if object.is_camera_target() {
                
                let position = object.get_center_position();
                drawing_manager.set_camera_target(
                    position.x,
                    position.y
                );
            }
        }
    }

    fn cull_dead_objects(&mut self) {
        for object_index in (0..self.game_objects.iter().count()).rev() {
            let object = self.game_objects.get(object_index).unwrap();

            if !object.is_alive() {
                self.game_objects.remove(object_index);
            }
        }
    }

    pub fn add_object(&mut self, object: GameObject){
        self.game_objects.push(object);
    }
}

