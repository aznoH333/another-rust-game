use crate::engine::{objects::{game_object_controller::GameObjectController, object_update::ObjectUpdate}, types::object_event::ObjectEvent};

pub struct ProjectileController {
    direction: f32,
    speed: f32,

}

impl ProjectileController {
    pub fn new(direction: f32, speed: f32) -> ProjectileController {
        return ProjectileController{
            direction,
            speed
        };
    }
}


impl GameObjectController for ProjectileController {
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, update: &mut ObjectUpdate) {
        core.x_velocity = self.direction.cos() * self.speed;
        core.y_velocity = self.direction.sin() * self.speed;
        core.rotation = self.direction;
    }
}