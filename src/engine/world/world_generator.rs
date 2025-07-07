use crate::engine::events::event_manager::{EventManager};

use super::world_manager::WorldManager;

pub trait WorldGenerator{
    fn generate_world(&mut self, world: &mut WorldManager, event_manager: &mut EventManager);
}