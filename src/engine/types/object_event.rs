use crate::engine::{objects::game_object::GameObject, types::controller_type::CONTROLLER_TYPE_OBJECT_COLLIDE};


pub struct WorldCollisionEvent {
    tile_x: i32, // TOOD : implement this
    tile_y: i32,
}

pub struct ObjectCollisionEvent {
    pub faction: u32,
    pub x: f32,
    pub y: f32,
    pub damage: f32,
}

pub struct ObjectEvent {
    pub event_type: u8,

    // event data
    pub world_collision: WorldCollisionEvent,
    pub object_collision: ObjectCollisionEvent, // TODO rewrite as options
}


impl ObjectEvent {
    pub fn new(event_type: u8) -> ObjectEvent{
        return ObjectEvent { 
            event_type: event_type, 
            world_collision: WorldCollisionEvent {
                tile_x: 0,
                tile_y: 0
            },
            object_collision: ObjectCollisionEvent { 
                faction: 0, 
                x: 0.0, 
                y: 0.0, 
                damage: 0.0 
            }
        };
    }

    pub fn new_object_collision(other: &GameObject) -> ObjectEvent {
        let mut event = ObjectEvent::new(CONTROLLER_TYPE_OBJECT_COLLIDE);
        event.object_collision.x = other.get_x();
        event.object_collision.y = other.get_y();
        event.object_collision.faction = other.get_faction();
        event.object_collision.damage = other.get_damage();
        return event;
    }
}