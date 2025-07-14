use std::time::SystemTime;
use std::time::UNIX_EPOCH;


pub struct Time {
    last_activated: u128,
}

impl Time {
    pub fn new() -> Time {
        return Time {
            last_activated: 0
        };
    }

    pub fn get_time_since_last_activation(&self) -> u128 {
        return Time::get_current_time() - self.last_activated;
    }

    pub fn activate(&mut self) {
        self.last_activated = Time::get_current_time();
    }

    pub fn get_current_time() -> u128 {
        return SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should go forward").as_millis();
    }
}