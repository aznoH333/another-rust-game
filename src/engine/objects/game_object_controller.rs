use crate::engine::{events::event_manager::{EventManager}, input::input::InputHandler};

use super::game_object_core::GameObjectCore;


pub trait GameObjectController{
    fn update(&mut self, core: &mut GameObjectCore, input: &InputHandler, event_manager: &mut EventManager);
}