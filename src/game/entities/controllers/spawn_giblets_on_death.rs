use ggez::graphics::Color;

use crate::{engine::objects::{game_object_controller::GameObjectController, object_summon::ObjectSummon}, game::entities::objects::effects::giblet_type, utils::{number_utils::NumberUtils, vec_utils::VecUtils}};
use crate::engine::events::game_event::GameEvent;
pub struct SpawnGibletsOnDeathController {
    giblet_type: u32,
    giblet_min: u32,
    giblet_max: u32,
    giblet_colors: Vec<Color>,
}

impl SpawnGibletsOnDeathController {
    pub fn new(giblet_type: u32, giblet_min: u32, giblet_max: u32, giblet_colors: Vec<Color>) -> SpawnGibletsOnDeathController {
        return SpawnGibletsOnDeathController { giblet_type: giblet_type, giblet_min, giblet_max, giblet_colors };
    }
}

impl GameObjectController for SpawnGibletsOnDeathController {
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, engine: &mut crate::engine::objects::object_update::ObjectUpdate) {
        let count = NumberUtils::random_integer(self.giblet_min as i32, self.giblet_max as i32);

        for _ in 0..count {
            let color = VecUtils::pick_random_element_vec::<Color>(&self.giblet_colors).to_owned();
            
            engine.event_manager.push_event(GameEvent::SpawnObject { summon: 
                ObjectSummon::new("giblet", core.x, core.y) 
                .set_color(color)
                .set_type(self.giblet_type)
                .set_speed(2.1)
            });
        }
    }
}