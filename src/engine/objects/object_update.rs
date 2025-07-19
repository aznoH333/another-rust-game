use crate::engine::types::game_update::GameUpdate;
use crate::engine::ui::ui_manager::UIManager;
use crate::engine::{events::event_manager::EventManager, input::input::InputHandler, world::world_manager::WorldManager};
use crate::engine::types::object_event::ObjectEvent;

pub struct ObjectUpdate<'a>{
    pub event: &'a ObjectEvent<'a>,
    pub input: &'a InputHandler,
    pub event_manager: &'a mut EventManager,
    pub world: &'a WorldManager,
    pub ui_manager: &'a mut UIManager
}

impl <'a>ObjectUpdate<'a> {
    pub fn new(event: &'a ObjectEvent<'a>, game_update: &'a mut GameUpdate) -> ObjectUpdate<'a> {
        return ObjectUpdate { 
            event: event, 
            input: game_update.input, 
            event_manager: game_update.event_manager, 
            world: game_update.world, 
            ui_manager: game_update.ui_manager, 
        }
    }
}