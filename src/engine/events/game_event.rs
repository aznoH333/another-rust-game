use crate::engine::objects::spawning::object_summon::ObjectSummon;

pub enum GameEvent{
    SpawnObject {
        summon: ObjectSummon,
    }
}