use crate::{engine::objects::game_object::{GameObject, GameObjectBuilder}, game::{entities::controllers::projectile_controller::ProjectileController, enums::drawing_layers::DrawingLayer}};

pub struct Bullet{}

impl Bullet {
    pub fn new(x: f32, y: f32, direction: f32, sprite: &str, speed: f32) -> GameObject {
        return GameObjectBuilder::new(x, y, sprite, DrawingLayer::WorldObjects.get_value())
        .set_dimensions(10.0, 10.0)
        .set_sprite_offset(-3.0, -3.0)
        .add_controller(Box::new(ProjectileController::new(direction, speed))).build();
    }
}