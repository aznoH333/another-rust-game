use crate::engine::objects::game_object::GameObject;


pub type ObjectSummonFunc = fn(&ObjectSummonParameters) -> GameObject;
pub struct ObjectSummonRegistration {
    pub summon_id: &'static str,
    pub summon_function: ObjectSummonFunc
}

impl ObjectSummonRegistration {
    pub const fn new(summon_id: &'static str, summon_function: ObjectSummonFunc) -> ObjectSummonRegistration {
        return ObjectSummonRegistration {
            summon_id,
            summon_function
        };
    }
}

inventory::collect!(ObjectSummonRegistration);

// TODO rename this to something that makes sense and sounds less esoteric
pub struct  ObjectSummonParameters{
    pub object_id: &'static str,
    pub x: f32,
    pub y: f32, // TODO : add more parameters (direction velocity faction etc)
}


pub struct ObjectSummon{
    parameters: ObjectSummonParameters
}

impl ObjectSummon {
    pub fn new(object_id: &'static str, x: f32, y: f32) -> ObjectSummon {
        return ObjectSummon{
            parameters: ObjectSummonParameters{
                object_id: object_id,
                x: x,
                y: y
            }
        };
    }

    pub fn build(&self) -> &ObjectSummonParameters {
        return &self.parameters;
    }
}


