use crate::engine::{self, objects::{game_object_controller::GameObjectController, object_update::ObjectUpdate}};

pub struct DestroyOnEvent{

}

impl DestroyOnEvent {
    pub fn new() -> DestroyOnEvent {
        return DestroyOnEvent {  };
    }
}

impl GameObjectController for DestroyOnEvent {
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, engine: &mut ObjectUpdate) {
        core.die();
    }
}