
use crate::engine::{drawing::drawing_manager::DrawingManager, events::event_manager::{EventManager}, input::input::InputHandler, objects::{object_simplification::{ObjectSimplification}, object_summon::{ObjectSummonParameters, ObjectSummonRegistration}, object_update::ObjectUpdate}, types::{controller_type::{CONTROLLER_TYPE_DESTROYED, CONTROLLER_TYPE_OBJECT_COLLIDE, CONTROLLER_TYPE_UPDATE, CONTROLLER_TYPE_WORLD_COLLIDE}, object_event::ObjectEvent}, world::world_manager::WorldManager};

use super::{game_object::GameObject, game_object_controller::GameObjectController};
use std::collections::HashMap;
use crate::engine::objects::object_summon::ObjectSummonFunc;
pub struct GameObjectManager{
    game_objects: Vec<GameObject>,
    object_summons: HashMap<String, ObjectSummonFunc>
}

impl GameObjectManager{
    pub fn new() -> GameObjectManager{
        return GameObjectManager{
            game_objects: Vec::new(),
            object_summons: HashMap::new()
        }
    }


    pub fn draw_objects(&mut self, drawing_manager: &mut DrawingManager) {
        for object in &mut self.game_objects {
            object.draw(drawing_manager);
        }
    }

    pub fn update(&mut self, drawing_manager: &mut DrawingManager, input: &InputHandler, world: &WorldManager, event_manager: &mut EventManager, delta: f32) {
        let object_simplifications = self.collect_object_simplifications();

        self.update_objects(input, world, event_manager, delta, object_simplifications);
        self.cull_dead_objects(input, event_manager, world); // these values should be somehow grouped instead of passing them around
        self.update_camera(drawing_manager);
        self.update_object_collisions(input, event_manager, world);

    }

    fn update_objects(&mut self, input: &InputHandler, world: &WorldManager, event_manager: &mut EventManager, delta: f32, object_simplifications: Vec<ObjectSimplification>) {
        // TODO : thanks to object simplifications. update objects and object collisions should now be merged into 1 loop        
        for object in &mut self.game_objects{
            // regular update
            let target = object.get_target().as_ref();
            let mut found_target: Option<&ObjectSimplification> = None;
            if target.is_some() {
                found_target = object_simplifications.iter().find(|a| a.name == *target.unwrap());
            }
            
            // TODO simlplify calling events
            let object_update_event = ObjectEvent::new_with_object(CONTROLLER_TYPE_UPDATE, found_target);
            let mut update_value = ObjectUpdate{
                event: &object_update_event, // inlining this will make rust shit the bed and cry
                input,
                event_manager,
                world
            };
            object.activate_event(&mut update_value);
            
            if object.collided_with_world() {
                let cool_rust_feature = ObjectEvent::new(CONTROLLER_TYPE_WORLD_COLLIDE);
                let mut object_update = GameObjectManager::get_engine_update(input, event_manager, world, &cool_rust_feature);

                object.activate_event(&mut object_update);
            }
            
            // update core
            object.update(world, delta);
        }
    }

    pub fn collect_object_simplifications(&self) -> Vec<ObjectSimplification> {
        let mut output = Vec::<ObjectSimplification>::new();

        for object in &self.game_objects {
            output.push(ObjectSimplification::new(object.get_core()));
        }

        return output;
    }

    fn update_object_collisions(&mut self, input: &InputHandler, event_manager: &mut EventManager, world_manager: &WorldManager) {
        let count = self.game_objects.iter().count();
        
        for object_index in 0..count {

            let object = self.game_objects.get(object_index).unwrap();
            

            for other_index in 0..count {
                if other_index == object_index {
                    continue;
                }

                let other_object = self.game_objects.get(other_index).unwrap();

                if self.game_objects.get(object_index).unwrap().collides_with_object(self.game_objects.get(other_index).unwrap()) {
                    let cool_rust_feature = ObjectSimplification::new(self.game_objects.get(other_index).unwrap().get_core()); 
                    // fire collision event
                    let object_update_event = ObjectEvent::new_with_object(CONTROLLER_TYPE_OBJECT_COLLIDE, 
                        Some(&cool_rust_feature));
                    let object_mut = self.game_objects.get_mut(object_index).unwrap();

                    let mut update_value = GameObjectManager::get_engine_update(input, event_manager, world_manager, &object_update_event);


                    object_mut.activate_event(&mut update_value);
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

    fn get_engine_update<'a>(input: &'a InputHandler, event_manager: &'a mut EventManager, world: &'a WorldManager, event: &'a ObjectEvent) -> ObjectUpdate<'a> {
        return ObjectUpdate { event, input, event_manager, world }
    }

    fn cull_dead_objects(&mut self, input: &InputHandler, event_manager: &mut EventManager, world: &WorldManager) {
        for object_index in (0..self.game_objects.iter().count()).rev() {
            let object = self.game_objects.get_mut(object_index).unwrap();

            if !object.is_alive() {
                let object_update_event = ObjectEvent::new(CONTROLLER_TYPE_DESTROYED);
                let mut update_value = ObjectUpdate{
                    event: &object_update_event, // inlining this will make rust shit the bed and cry
                    input,
                    event_manager,
                    world
                };
                object.activate_event(&mut update_value);
                
                self.game_objects.remove(object_index);
            }
        }
    }

    pub fn add_object(&mut self, object: GameObject){
        self.game_objects.push(object);
    }

    pub fn register_summon(&mut self, register_command: &ObjectSummonRegistration) {
        self.object_summons.insert(register_command.summon_id.to_owned(), register_command.summon_function);
    }

    pub fn summon_object(&mut self, summon: &ObjectSummonParameters) {
        let summon_ref = self.object_summons.get(&summon.object_id.to_owned())
            .expect(format!("tried to summon object with unknown id : {}", summon.object_id).as_str());

        self.add_object(summon_ref(summon));

    }
}

