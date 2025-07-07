use crate::engine::objects::object_summon::ObjectSummon;

pub enum GameEvent{
    SpawnObject {
        summon: ObjectSummon,
    }
}