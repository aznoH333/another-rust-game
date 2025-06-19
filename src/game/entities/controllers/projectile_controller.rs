use crate::engine::objects::game_object_controller::GameObjectController;

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
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, input: &crate::engine::input::input::InputHandler, event_manager: &mut crate::engine::events::event_manager::EventManager) {
        core.x_velocity = self.direction.cos() * self.speed;
        core.y_velocity = self.direction.sin() * self.speed;
    }
}