use std::time::UNIX_EPOCH;
use std::time::SystemTime;
pub struct Timer{
    last_fired: u128,
    timeout: u128,
}

impl Timer {
    pub fn new(timeout: u128) -> Timer {
        return Timer { last_fired: 0, timeout };
    }

    pub fn can_activate(&self) -> bool {
        return self.get_current_time() - self.last_fired >= self.timeout;
    }

    pub fn activate(&mut self) {
        self.last_fired = self.get_current_time();
    }

    fn get_current_time(&self) -> u128 {
        return SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should go forward").as_millis();
    }

    pub fn set_cooldown(&mut self, cooldown: u128) {
        self.timeout = cooldown;
    }

    pub fn get_as_percentage(&self) -> f32 {
        return (self.get_current_time() as f32 - self.last_fired as f32) / self.timeout as f32;
    }
}