use crate::{engine::{objects::game_object::{GameObject, GameObjectBuilder}, types::controller_type::{CONTROLLER_TYPE_OBJECT_COLLIDE, CONTROLLER_TYPE_UPDATE, CONTROLLER_TYPE_WORLD_COLLIDE}}, game::{entities::controllers::{destroy_on_event::DestroyOnEvent, destroy_on_hostile_collision::DestroyOnHostileCollision, projectile_controller::ProjectileController}, enums::drawing_layers::DrawingLayer}};

pub struct Bullet{}

impl Bullet {
    pub fn new(x: f32, y: f32, direction: f32, sprite: &str, speed: f32, faction: u32, damage: f32) -> GameObject {
        return GameObjectBuilder::new(x, y, sprite, DrawingLayer::WorldObjects.get_value())
        .set_dimensions(2.0, 2.0)
        .set_sprite_offset(-7.0, -7.0)
        .set_faction(faction)
        .set_damage(damage)
        .set_name("projectile")
        .disable_auto_flipping()
        .add_controller(CONTROLLER_TYPE_UPDATE,Box::new(ProjectileController::new(direction, speed)))
        .add_controller(CONTROLLER_TYPE_WORLD_COLLIDE, Box::new(DestroyOnEvent::new()))
        .add_controller(CONTROLLER_TYPE_OBJECT_COLLIDE, Box::new(DestroyOnHostileCollision::new()))
        .build();
    }
}