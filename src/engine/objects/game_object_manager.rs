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
        let mut camera_target: Option<&GameObject> = None;

        for object in &mut self.game_objects{
            object.update(drawing_manager, input, world, event_manager);

            if object.is_camera_target(){
                camera_target = Some(object);
            }
        }

        if camera_target.is_some() {
            let target = camera_target.unwrap();
            drawing_manager.set_camera_target(
                target.get_x() + target.get_width() / 2.0, 
                target.get_y() + target.get_height() / 2.0,
            );
        }
    }
    
    pub fn add_object(&mut self, object: GameObject){
        self.game_objects.push(object);
    }
}

