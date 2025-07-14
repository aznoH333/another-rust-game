use crate::engine::{objects::game_object_controller::GameObjectController, utils::timer::Timer};

pub struct FadeAwayController {
    fade_timer: Timer
}

impl FadeAwayController {
    pub fn new(fade_timer: u128) -> FadeAwayController{
        let mut out = FadeAwayController { fade_timer: Timer::new(fade_timer) };
        out.fade_timer.activate();
        return out;
    }
}

impl GameObjectController for FadeAwayController {
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, engine: &mut crate::engine::objects::object_update::ObjectUpdate) {
        core.sprite.get_color_mut().a = 1.0 - self.fade_timer.get_as_percentage();
        if self.fade_timer.can_activate() {
            core.die();
        }
    }
}