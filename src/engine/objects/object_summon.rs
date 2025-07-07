use crate::{engine::objects::game_object::GameObject, game::entities::factions::FACTION_NEUTRAL};


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
    pub y: f32,
    pub damage: f32,
    pub faction: u32,
    pub direction: f32,
    pub sprite: &'static str,
    pub speed: f32,
    pub object_type: u32,
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
                y: y,
                damage: 0.0,
                faction: FACTION_NEUTRAL,
                direction: 0.0,
                sprite: "undefined",
                speed: 0.0,
                object_type: 0,
            }
        };
    }

    pub fn set_damage(mut self, damage: f32) -> ObjectSummon {
        self.parameters.damage = damage;
        return self;
    }

    pub fn set_faction(mut self, faction: u32) -> ObjectSummon {
        self.parameters.faction = faction;
        return self;
    }

    pub fn set_sprite(mut self, sprite: &'static str) -> ObjectSummon {
        self.parameters.sprite = sprite;
        return self;
    }

    pub fn set_direction(mut self, direction: f32) -> ObjectSummon {
        self.parameters.direction = direction;
        return self;
    }

    pub fn set_speed(mut self, speed: f32) -> ObjectSummon {
        self.parameters.speed = speed;
        return self;
    }

    pub fn set_type(mut self, object_type: u32) -> ObjectSummon {
        self.parameters.object_type = object_type;
        return self;
    }

    pub fn build(&self) -> &ObjectSummonParameters {
        return &self.parameters;
    }
}


