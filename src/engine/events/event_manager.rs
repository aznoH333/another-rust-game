use crate::engine::objects::game_object_manager::GameObjectManager;

use super::game_event::GameEvent;

pub struct EventManager{
    events: Vec<GameEvent>
}


impl EventManager {
    pub fn new() -> EventManager {
        return EventManager{
            events: Vec::new()
        }
    }

    pub fn push_event(&mut self, event: GameEvent){
        self.events.push(event);
    }


    pub fn update_events(&mut self, game_object_manager: &mut GameObjectManager){
        for event in &self.events{
            if let GameEvent::SpawnObject { summon } = event {
                game_object_manager.summon_object(&summon.build());
            }
        }

        self.events.clear();
    }
}