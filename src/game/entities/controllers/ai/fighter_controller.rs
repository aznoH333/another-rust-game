use ggez::glam::Vec2;

use crate::{engine::{events::game_event::GameEvent, objects::{game_box::GameBox, game_object_controller::GameObjectController, game_object_core::GameObjectCore, object_summon::ObjectSummon, object_update::ObjectUpdate}, utils::timer::Timer, world::world_constants::TILE_SIZE}, game::entities::controllers::ai::ai_state::{self, AI_STATE_ALERTED, AI_STATE_FIGHTING, AI_STATE_IDLE}, utils::space_utils::SpaceUtils};

pub struct FighterController {
    target_name: String,
    ai_state: u8,
    last_target_pos: Vec2,
    has_line_of_sight_to_target: bool,
    has_line_of_sight_to_last_pos: bool,
    last_pos_exists: bool,
    attentions_span: Timer,
}

impl FighterController {
    pub fn new (target_name: &str) -> FighterController {
        return FighterController {
            target_name: target_name.to_owned(),
            ai_state: AI_STATE_IDLE,
            last_target_pos: Vec2 { x: 0.0, y: 0.0 },
            has_line_of_sight_to_last_pos: false,
            has_line_of_sight_to_target: false,
            last_pos_exists: false,
            attentions_span: Timer::new(5000),
        };
    }

    fn switch_state(&mut self, core: &mut GameObjectCore, new_state: u8, engine: &mut ObjectUpdate) {
        if new_state == self.ai_state {
            return;
        }
        
        if self.ai_state == AI_STATE_IDLE {
            // spawn ! callout
            engine.event_manager.push_event(GameEvent::SpawnObject { summon: ObjectSummon::new("callout", core.x, core.top()).set_sprite("emotions_0001") });
        }else if new_state == AI_STATE_IDLE {
            // spawn ? callout
            engine.event_manager.push_event(GameEvent::SpawnObject { summon: ObjectSummon::new("callout", core.x, core.top()).set_sprite("emotions_0002") });
        }else if new_state == AI_STATE_ALERTED {
            // start timer
            self.attentions_span.activate();
        }

        self.ai_state = new_state;
    }

    //fn unalerted(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, engine: &mut ObjectUpdate) {

    //}
}

impl GameObjectController for FighterController {
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, engine: &mut ObjectUpdate) {
        if engine.event.found_object.is_some() {
            let other = engine.event.found_object.unwrap();

            self.has_line_of_sight_to_target = engine.world.has_line_of_sight(core.x, core.y, other.x, other.y);
            
            if self.has_line_of_sight_to_target {
                self.last_pos_exists = true;
                self.last_target_pos.x = other.x;
                self.last_target_pos.y = other.y;
            }
            
        }
        self.has_line_of_sight_to_last_pos = engine.world.has_line_of_sight(core.x, core.y, self.last_target_pos.x, self.last_target_pos.y);


        // update state
        if self.has_line_of_sight_to_target {
            self.switch_state(core, AI_STATE_FIGHTING, engine);
        }else if self.last_pos_exists {
            self.switch_state(core, AI_STATE_ALERTED, engine);
        }else {
            self.switch_state(core, AI_STATE_IDLE, engine);
        }


        // delete last target if reached
        if self.last_pos_exists && 
        !self.has_line_of_sight_to_target && 
        SpaceUtils::pythagoras(core.x, core.y, self.last_target_pos.x, self.last_target_pos.y) < TILE_SIZE as f32 {
            self.last_pos_exists = false;
        }

        // delete last target if attention ran out
        if self.ai_state == AI_STATE_ALERTED && self.attentions_span.can_activate() {
            self.last_pos_exists = false;
        }

        if self.has_line_of_sight_to_last_pos && self.last_pos_exists {
            let direction = SpaceUtils::direction_towards(core.x, core.y, self.last_target_pos.x, self.last_target_pos.y);
            core.movement_x = direction.cos();
            core.movement_y = direction.sin();

            if core.collided_with_world() {
                core.movement_x = core.movement_x.signum();
                core.movement_y = core.movement_y.signum();
            }
        }
        
    }

    

    
}