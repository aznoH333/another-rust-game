use crate::engine::objects::object_update::ObjectUpdate;

use super::game_object_core::GameObjectCore;

pub trait GameObjectController{
    fn update(&mut self, core: &mut GameObjectCore, engine: &mut ObjectUpdate);
}