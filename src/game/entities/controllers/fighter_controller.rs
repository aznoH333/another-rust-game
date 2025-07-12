use crate::{engine::{events::game_event::GameEvent, objects::{game_box::GameBox, game_object_controller::GameObjectController, object_summon::ObjectSummon, object_update::ObjectUpdate}}, utils::space_utils::SpaceUtils};

pub struct FighterController {
    target_name: String,
    alerted: bool,
}

impl FighterController {
    pub fn new (target_name: &str) -> FighterController {
        return FighterController {
            target_name: target_name.to_owned(),
            alerted: false,
        };
    }
}

impl GameObjectController for FighterController {
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, engine: &mut ObjectUpdate) {
        if engine.event.found_object.is_some() {
            let other = engine.event.found_object.unwrap();

            if !engine.world.has_line_of_sight(core.x, core.y, other.x, other.y) {
                return;
            }

            // alert
            if !self.alerted {
                self.alerted = true;
                engine.event_manager.push_event(GameEvent::SpawnObject { summon: ObjectSummon::new("callout", core.x, core.top()).set_sprite("emotions_0001") });
            }


            let direction = SpaceUtils::direction_towards(core.x, core.y, other.x, other.y);
            core.movement_x = direction.cos();
            core.movement_y = direction.sin();
        }        
    }
}