
use crate::engine::{drawing::drawing_manager::DrawingManager, events::event_manager::{self, EventManager}, input::input::InputHandler, performance_monitoring::performance_monitor::PerformanceMonitor, types::{controller_type::{CONTROLLER_TYPE_OBJECT_COLLIDE, CONTROLLER_TYPE_UPDATE, CONTROLLER_TYPE_WORLD_COLLIDE}, object_event::ObjectEvent}, world::world_manager::WorldManager};

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


    pub fn draw_objects(&mut self, drawing_manager: &mut DrawingManager) {
        for object in &mut self.game_objects {
            object.draw(drawing_manager);
        }
    }

    pub fn update(&mut self, drawing_manager: &mut DrawingManager, input: &InputHandler, world: &WorldManager, event_manager: &mut EventManager, delta: f32) {

        self.update_objects(input, world, event_manager, delta);
        self.cull_dead_objects();
        self.update_camera(drawing_manager);
        self.update_object_collisions(input, event_manager);

    }

    fn update_objects(&mut self, input: &InputHandler, world: &WorldManager, event_manager: &mut EventManager, delta: f32) {
        for object in &mut self.game_objects{
            // regular update
            let object_update_event = ObjectEvent::new(CONTROLLER_TYPE_UPDATE);
            object.activate_event(&object_update_event, input, event_manager);
            
            if object.collided_with_world() {
                object.activate_event(&ObjectEvent::new(CONTROLLER_TYPE_WORLD_COLLIDE), input, event_manager);
            }
            
            // update core
            object.update(world, delta);
        }
    }

    fn update_object_collisions(&mut self, input: &InputHandler, event_manager: &mut EventManager) {
        let count = self.game_objects.iter().count();
        
        for object_index in 0..count {

            let object = self.game_objects.get(object_index).unwrap();
            

            for other_index in 0..count {
                if other_index == object_index {
                    continue;
                }

                let other_object = self.game_objects.get(other_index).unwrap();

                if self.game_objects.get(object_index).unwrap().collides_with_object(self.game_objects.get(other_index).unwrap()) {
                    // fire collision event
                    let object_update_event = ObjectEvent::new_object_collision(self.game_objects.get(other_index).unwrap());
                    let mut object_mut = self.game_objects.get_mut(object_index).unwrap();
                    object_mut.activate_event(&object_update_event, input, event_manager);
                }
            }
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

