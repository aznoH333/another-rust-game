
pub struct WorldCollisionEvent {
    tile_x: i32, // TOOD : implement this
    tile_y: i32,
}

pub struct ObjectEvent {
    pub event_type: u8,

    // event data
    pub world_collision: WorldCollisionEvent,
}


impl ObjectEvent {
    pub fn new(event_type: u8) -> ObjectEvent{
        return ObjectEvent { 
            event_type: event_type, 
            world_collision: WorldCollisionEvent {
                tile_x: 0,
                tile_y: 0
            } 
        };
    }
}