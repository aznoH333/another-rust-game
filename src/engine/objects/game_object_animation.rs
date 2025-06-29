pub struct GameObjectAnimation {
    frames: Vec::<String>,
    animation_speed: f32,
    current_time: f32,
    current_frame: u32,
    frame_count: u32,
}


impl GameObjectAnimation {
    pub fn new(animation_speed: f32) -> GameObjectAnimation {
        return GameObjectAnimation{
            frames: Vec::new(),
            animation_speed,
            current_time: 0.0,
            current_frame: 0,
            frame_count: 0,
        };
    }

    pub fn add_frame(mut self, frame: &str) -> GameObjectAnimation {
        self.frames.push(frame.to_owned());
        self.frame_count += 1;
        return self;
    }

    pub fn reset_animation(&mut self) {
        self.current_time = 0.0;
    }

    pub fn update_animation(&mut self, delta: f32) {
        self.current_time += delta;
        if self.current_time >= self.animation_speed {
            self.current_frame += 1;
            
            if self.current_frame >= self.frame_count {
                self.current_frame = 0;
            }
            
            self.current_time -= self.animation_speed;
        }
    }

    pub fn get_current_frame(&self) -> &String {
        return self.frames.get(self.current_frame as usize).unwrap();
    }
}