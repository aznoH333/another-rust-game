use crate::{engine::{objects::game_object::{GameObject, GameObjectBuilder}, types::controller_type::{CONTROLLER_TYPE_OBJECT_COLLIDE, CONTROLLER_TYPE_UPDATE}}, game::{entities::controllers::{fighter_controller::FighterController, take_damage_on_hostile_collision::TakeDamageOnHostileCollisionController}, enums::drawing_layers::DrawingLayer}};
use crate::game::entities::factions::FACTION_ENEMY;

pub struct Gremlin {

}

impl Gremlin {
    pub fn new(x: f32,y: f32) -> GameObject {
        return 
            GameObjectBuilder::new(x, y, "gremlin_0001", DrawingLayer::GameObjects.get_value())
            .set_dimensions(10.0, 10.0)
            .set_sprite_offset(-3.0, -6.0)

            // stats
            .set_speed(0.65)

            // combat
            .set_faction(FACTION_ENEMY)
            .set_health(25.0)
            .set_name("gremlin")
            .set_target("player")

            // controllers
            .add_controller(CONTROLLER_TYPE_OBJECT_COLLIDE, Box::new(TakeDamageOnHostileCollisionController::new(200)))
            .add_controller(CONTROLLER_TYPE_UPDATE, Box::new(FighterController::new("player")))

            .build()
            ;
    }
}