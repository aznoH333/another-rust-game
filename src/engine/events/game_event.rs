use crate::engine::objects::{game_object_manager::GameObjectManager, object_summon::ObjectSummon};

pub enum GameEvent{
    SpawnObject {
        summon: ObjectSummon,
    }
}