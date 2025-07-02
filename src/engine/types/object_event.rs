use crate::engine::{objects::game_object::GameObject, types::controller_type::{CONTROLLER_TYPE_OBJECT_COLLIDE, CONTROLLER_TYPE_UPDATE}};

pub struct ObjectCollisionEvent {
    pub faction: u32,
    pub x: f32,
    pub y: f32,
    pub damage: f32,
}

pub struct ObjectSearchResult {
    pub x: f32,
    pub y: f32,
}

pub struct ObjectEvent {
    pub event_type: u8,

    // event data
    pub object_collision: Option<ObjectCollisionEvent>,
    pub object_search_result: Option<ObjectSearchResult>,
}


impl ObjectEvent {
    pub fn new(event_type: u8) -> ObjectEvent{
        return ObjectEvent { 
            event_type: event_type, 
            object_collision: None,
            object_search_result: None,
        };
    }

    pub fn new_object_collision(other: &GameObject) -> ObjectEvent {
        let mut event = ObjectEvent::new(CONTROLLER_TYPE_OBJECT_COLLIDE);
        event.object_collision = Some(ObjectCollisionEvent{
            x: other.get_x(),
            y: other.get_y(),
            faction: other.get_faction(),
            damage: other.get_damage(),
        });
        return event;
    }

    pub fn new_object_search_result(other: Option<&GameObject>) -> ObjectEvent {
        let mut event = ObjectEvent::new(CONTROLLER_TYPE_UPDATE);
        
        if other.is_some() {
            let other_u = other.unwrap();
            event.object_search_result = Some(ObjectSearchResult{
                x: other_u.get_x(),
                y: other_u.get_y(),
            });
        }

        return event;
    }
}