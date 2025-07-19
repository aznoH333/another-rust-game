use crate::engine::events::event_manager::EventManager;
use crate::engine::input::input::InputHandler;
use crate::engine::ui::ui_manager::UIManager;
use crate::engine::world::world_manager::WorldManager;

pub struct GameUpdate<'a>{
    pub input: &'a InputHandler,
    pub event_manager: &'a mut EventManager,
    pub world: &'a WorldManager,
    pub ui_manager: &'a mut UIManager,
    pub delta: f32,
}

impl <'a>GameUpdate<'a>{
    pub fn new(input: &'a InputHandler, event_manager: &'a mut EventManager, world: &'a WorldManager, ui_manager: &'a mut UIManager, delta: f32) -> GameUpdate<'a> {
        return GameUpdate { 
            input: input, 
            event_manager: event_manager, 
            world: world, 
            ui_manager: ui_manager, 
            delta: delta,
        }
    }
}