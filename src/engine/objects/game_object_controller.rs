use crate::engine::{events::event_manager::EventManager, input::input::InputHandler, objects::object_update::ObjectUpdate, types::object_event::ObjectEvent};

use super::game_object_core::GameObjectCore;

pub trait GameObjectController{
    fn update(&mut self, core: &mut GameObjectCore, engine: &mut ObjectUpdate);
}