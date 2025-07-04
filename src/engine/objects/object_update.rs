use crate::engine::{events::event_manager::EventManager, input::input::InputHandler, world::world_manager::WorldManager};
use crate::engine::types::object_event::ObjectEvent;

pub struct ObjectUpdate<'a>{
    pub event: &'a ObjectEvent<'a>,
    pub input: &'a InputHandler,
    pub event_manager: &'a mut EventManager,
    pub world: &'a WorldManager,
}