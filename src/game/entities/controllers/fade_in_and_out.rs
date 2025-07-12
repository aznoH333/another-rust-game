use crate::engine::{objects::game_object_controller::GameObjectController, utils::timer::Timer};

pub struct FadeInAndOut{
    timer: Timer,
    fade_in_time: u128,
    fade_out_time: u128
}

impl FadeInAndOut {
    pub fn new(total_time: u128, fade_in_time: u128, fade_out_time: u128) -> FadeInAndOut {
        let mut out=  FadeInAndOut { 
            timer: Timer::new(total_time), 
            fade_in_time: fade_in_time, 
            fade_out_time: fade_out_time 
        };

        out.timer.activate();

        return out;
    }
}

impl GameObjectController for FadeInAndOut {
    fn update(&mut self, core: &mut crate::engine::objects::game_object_core::GameObjectCore, engine: &mut crate::engine::objects::object_update::ObjectUpdate) {
        let progress = self.timer.get_as_number();

        if self.timer.can_activate() {
            core.die();
            return;
        }

        if progress < self.fade_in_time {
            core.color.a = (progress as f32) / (self.fade_in_time as f32);
        } else if progress > self.timer.get_cooldown() - self.fade_out_time {
            core.color.a = ((self.timer.get_cooldown() - progress) as f32) / (self.fade_out_time as f32);
        } else {
            core.color.a = 1.0;
        }
        
        
    }
}