use crate::engine::objects::game_object_manager::GameObjectManager;

pub enum GameEvent{
    SpawnObject {spawn_function: Box<dyn Fn(&mut GameObjectManager)>}
}