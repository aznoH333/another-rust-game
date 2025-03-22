use crate::engine::drawing::drawing_manager::{self, DrawingManager};

use super::game_object_core::GameObjectCore;


pub trait GameObjectController{
    fn update(&mut self, core: &mut GameObjectCore);
}