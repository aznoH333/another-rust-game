use crate::{engine::{objects::game_object::{GameObject, GameObjectBuilder}, types::controller_type::CONTROLLER_TYPE_OBJECT_COLLIDE}, game::{entities::controllers::take_damage_on_hostile_collision::TakeDamageOnHostileCollisionController, enums::drawing_layers::DrawingLayer}};
use crate::game::entities::factions::FACTION_ENEMY;

pub struct Gremlin {

}

impl Gremlin {
    pub fn new(x: f32,y: f32) -> GameObject {
        return 
            GameObjectBuilder::new(x, y, "gremlin_0001", DrawingLayer::GameObjects.get_value())
            .set_dimensions(10.0, 10.0)
            .set_sprite_offset(-3.0, -6.0)

            // combat
            .set_faction(FACTION_ENEMY)
            .set_health(25.0)
            .set_name("gremlin")

            // controllers
            .add_controller(CONTROLLER_TYPE_OBJECT_COLLIDE, Box::new(TakeDamageOnHostileCollisionController::new(200)))

            .build()
            ;
    }
}