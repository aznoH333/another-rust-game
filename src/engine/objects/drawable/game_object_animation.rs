use crate::engine::utils::time::Time;

pub struct GameObjectAnimation {
    frames: Vec::<String>,
    frame_duration: u128,
    frame_count: u32,
    frame_timer: Time,
    loop_last_frame: bool,
}


impl GameObjectAnimation {
    pub fn new(frame_duration: u128) -> GameObjectAnimation {
        return GameObjectAnimation{
            frames: Vec::new(),
            frame_duration,
            frame_count: 0,
            frame_timer: Time::new(),
            loop_last_frame: false,
        };
    }

    pub fn add_frame(mut self, frame: &str) -> GameObjectAnimation {
        self.frames.push(frame.to_owned());
        self.frame_count += 1;
        return self;
    }

    pub fn make_last_frame_loop(mut self) -> GameObjectAnimation {
        self.loop_last_frame = true;
        return self;
    }

    pub fn reset_animation(&mut self) {
        self.frame_timer.activate();
    }

    pub fn get_current_frame(&self) -> &String {
        
        let frame_time = (self.frame_timer.get_time_since_last_activation() / self.frame_duration) as u32;
        let mut index = (frame_time % self.frame_count) as usize;
        if self.loop_last_frame && frame_time > self.frame_count {
            index = (self.frame_count - 1) as usize;
        }

        return self.frames.get(index).unwrap();
    }
}