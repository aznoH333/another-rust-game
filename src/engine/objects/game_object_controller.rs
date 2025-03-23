use crate::engine::{drawing::drawing_manager::{self, DrawingManager}, input::input::InputHandler};

use super::game_object_core::GameObjectCore;


pub trait GameObjectController{
    fn update(&mut self, core: &mut GameObjectCore, input: &InputHandler);
}