use crate::utils::vec_utils::pick_random_element_vec;

pub struct RoomGenerationPoint{
    allow_up: bool,
    allow_down: bool,
    allow_left: bool,
    allow_right: bool,
}

impl RoomGenerationPoint {
    pub fn new(allow_up: bool, allow_down: bool, allow_left: bool, allow_right: bool) -> RoomGenerationPoint {
        return RoomGenerationPoint { allow_up, allow_down, allow_left, allow_right }
    }
    
    pub fn get_valid_direction(&self) -> (i32, i32){
        let mut possible_outputs: Vec<(i32, i32)> = Vec::new();

        if self.allow_up { possible_outputs.push((0, -1)); }
        if self.allow_down { possible_outputs.push((0, 1)); }
        if self.allow_left { possible_outputs.push((-1, 0)); }
        if self.allow_right { possible_outputs.push((1, 0)); }
        
        return pick_random_element_vec(&possible_outputs).to_owned();
    }

    pub fn from_direction(direction: (i32, i32)) -> Self {
        if direction.0 != 0 {
            return RoomGenerationPoint{allow_up: true, allow_down: true, allow_left: false, allow_right: false };
        }
        return RoomGenerationPoint{allow_up: false, allow_down: false, allow_left: true, allow_right: true };
    }
}