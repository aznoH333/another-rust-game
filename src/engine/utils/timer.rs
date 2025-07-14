use crate::engine::utils::time::Time;
pub struct Timer{
    time: Time,
    timeout: u128,
}

impl Timer {
    pub fn new(timeout: u128) -> Timer {
        return Timer { 
            time: Time::new(),
            timeout 
        };
    }

    pub fn can_activate(&self) -> bool {
        return self.time.get_time_since_last_activation() >= self.timeout;
    }

    pub fn activate(&mut self) {
        self.time.activate();
    }


    pub fn set_cooldown(&mut self, cooldown: u128) {
        self.timeout = cooldown;
    }

    pub fn get_as_percentage(&self) -> f32 {
        return (self.time.get_time_since_last_activation()) as f32 / (self.timeout as f32);
    }

    pub fn get_as_number(&self) -> u128 {
        return self.time.get_time_since_last_activation();
    }

    pub fn get_cooldown(&self) -> u128 {
        return self.timeout;
    }
}