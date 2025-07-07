use crate::engine::objects::object_simplification::ObjectSimplification;

pub struct ObjectEvent<'a> {
    pub event_type: u8,

    // event data
    pub found_object: Option<&'a ObjectSimplification>,
}


impl<'a> ObjectEvent<'a> {
    pub fn new(event_type: u8) -> ObjectEvent<'a>{
        return ObjectEvent { 
            event_type: event_type, 
            found_object: None,
        };
    }

    pub fn new_with_object(event_type: u8, other: Option<&'a ObjectSimplification>) -> ObjectEvent<'a> {
        let mut event = ObjectEvent::new(event_type);
        event.found_object = other;
        return event;
    }

}