use crate::engine::drawing::drawing_manager::DrawingManager;

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


    pub fn update(&mut self, drawing_manager: &mut DrawingManager) {
        for object in &mut self.game_objects{
            object.update(drawing_manager);
        }
    }
    
    pub fn add_object(&mut self, object: GameObject){
        self.game_objects.push(object);
    }
}

