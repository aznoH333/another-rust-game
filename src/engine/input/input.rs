use ggez::input::keyboard::KeyCode;

pub struct InputHandler{
    // Keyboard events
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    action1: bool,
    action2: bool,
}

impl InputHandler{
    pub fn new() -> InputHandler{
        return InputHandler { 
            up: false, 
            down: false, 
            left: false, 
            right: false, 
            action1: false, 
            action2: false 
        };
    }

    pub fn reset_input_state(&mut self) {
        self.up = false;
        self.down = false;
        self.right = false;
        self.left = false;
        self.action1 = false;
        self.action2 = false;
    }

    pub fn handle_keyboard_down_events(&mut self, input: ggez::input::keyboard::KeyInput) {
        match input.keycode {
            Some(KeyCode::Up) => {
                self.up = true;
            }
            Some(KeyCode::Down) => {
                self.down = true;
            }
            Some(KeyCode::Left) => {
                self.left = true;
            }
            Some(KeyCode::Right) => {
                self.right = true;
            }
            Some(KeyCode::Z) => {
                self.action1 = true;
            }
            Some(KeyCode::X) => {
                self.action2 = true;
            }
            _=>{} // do nothing
        }
    }


    pub fn handle_keyboard_up_events(&mut self, input: ggez::input::keyboard::KeyInput) {
        match input.keycode {
            Some(KeyCode::Up) => {
                self.up = false;
            }
            Some(KeyCode::Down) => {
                self.down = false;
            }
            Some(KeyCode::Left) => {
                self.left = false;
            }
            Some(KeyCode::Right) => {
                self.right = false;
            }
            Some(KeyCode::Z) => {
                self.action1 = false;
            }
            Some(KeyCode::X) => {
                self.action2 = false;
            }
            _=>{} // do nothing
        }
    }


    pub fn key_down(&self) -> bool {
        return self.down;
    }

    pub fn key_up(&self) -> bool {
        return self.up;
    }

    pub fn key_left(&self) -> bool {
        return self.left;
    }

    pub fn key_right(&self) -> bool {
        return self.right;
    }

    pub fn key_action1(&self) -> bool {
        return self.action1;
    }

    pub fn key_action2(&self) -> bool {
        return self.action2;
    }
}