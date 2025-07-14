use crate::engine::world::world_constants::TILE_SIZE;

pub struct SpaceUtils{

}

impl SpaceUtils {
    pub fn squares_collide(x1: i32, y1: i32, w1: i32, h1: i32, x2: i32, y2: i32, w2: i32, h2: i32) -> bool {
        return 
            x1 + w1 >= x2 &&
            x1 < x2 + w2 &&
            y1 + h1 >= y2 &&
            y1 < y2 + h2;
    }

    pub fn squares_collide_f32(x1: f32, y1: f32, w1: f32, h1: f32, x2: f32, y2: f32, w2: f32, h2: f32) -> bool {
        return 
            x1 + w1 >= x2 &&
            x1 < x2 + w2 &&
            y1 + h1 >= y2 &&
            y1 < y2 + h2;
    }


    pub fn pythagoras(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        return ((x1 - x2).abs().powf(2.0) + (y1 - y2).abs().powf(2.0)).sqrt();
    }

    pub fn game_units_to_world_units(value: f32) -> i32{
        return (value / (TILE_SIZE as f32)) as i32;
    }

    pub fn world_units_to_game_units(value: i32) -> f32 {
        return (value * TILE_SIZE) as f32;
    }

    pub fn align_to_world_grid(value: f32) -> f32 {
        return (value / TILE_SIZE as f32).round() * TILE_SIZE as f32; //- (TILE_SIZE as f32 / 2.0);
    }

    pub fn direction_towards(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        return (y2 - y1).atan2(x2 - x1);
    }

    pub fn get_vec_length(x: f32, y: f32) -> f32 {
        return Self::pythagoras(0.0, 0.0, x, y);
    }
}
