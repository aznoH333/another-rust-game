use crate::{engine::{objects::{game_object_controller::GameObjectController, object_update::ObjectUpdate}, world}, utils::space_utils::SpaceUtils};

pub struct FighterController {
    target_name: String
}

impl FighterController {
    pub fn new (target_name: &str) -> FighterController {
        return FighterController {
            target_name: target_name.to_owned()
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

            let direction = SpaceUtils::direction_towards(core.x, core.y, other.x, other.y);
            core.movement_x = direction.cos();
            core.movement_y = direction.sin();
        }        
    }
}