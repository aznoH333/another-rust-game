use crate::engine::{events::event_manager::EventManager, input::input::InputHandler, types::object_event::ObjectEvent};

use super::game_object_core::GameObjectCore;


pub trait GameObjectController{
    fn update(&mut self, core: &mut GameObjectCore, event: &ObjectEvent, input: &InputHandler, event_manager: &mut EventManager);
}