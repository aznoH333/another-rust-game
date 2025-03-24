use crate::{engine::{drawing::drawing_manager::DrawingManager, objects::game_object_core::GameObjectCore}, game};

use super::{world_constants::TILE_SIZE, world_tile::WorldTile};

pub struct WorldManager{
    world: Vec<Vec<WorldTile>>
}

impl WorldManager{
    pub fn new() -> WorldManager {
        
        let mut this = WorldManager{
            world: Vec::new()
        };
        this.generate_test_world();
        return this;
    }

    pub fn generate_test_world(&mut self){
        self.prepare_world(10, 10);

        for y in 0..5{
            self.set_tile_properties(4, 2+y, "tiles_0001.png", true);
        }
    }

    fn prepare_world(&mut self, width: i32, height: i32){
        for x in 0..width{
            if self.world.get(x as usize).is_none() {
                self.world.insert(x as usize, Vec::new());
            }
            
            for y in 0..height{
                self.world.get_mut(x as usize).unwrap().insert(y as usize, 
                    WorldTile::new(false, "tiles_0005.png", x * TILE_SIZE, y * TILE_SIZE)
                );
            }
        }
    }

    fn get_tile_mut(&mut self, x: i32, y: i32) -> &mut WorldTile{
        return self.world.get_mut(x as usize).unwrap().get_mut(y as usize).unwrap();
    }

    fn get_tile(&self, x: i32, y: i32) -> &WorldTile{
        return self.world.get(x as usize).unwrap().get(y as usize).unwrap();
    }

    fn is_tile_solid(&self, x: i32, y: i32) -> bool {
        return self.get_tile(x, y).is_solid()
    }

    fn set_tile_properties(&mut self, x: i32, y: i32, texture: &str, is_solid: bool){
        let tile = self.get_tile_mut(x, y);
        tile.set_texture(texture);
        tile.set_solid(is_solid);
    }

    pub fn draw_world(&mut self, drawing: &mut DrawingManager){
        for v in &self.world {
            for tile in v{
                tile.draw(drawing);
            }
        }
    }

    pub fn move_in_world(&self, game_object: &mut GameObjectCore) {
        // x move
        let x_colider = self.check_world_square_collisions(
            game_object.x + game_object.x_velocity, 
            game_object.y, 
            game_object.width, 
            game_object.height);

        if x_colider.is_none(){
            game_object.x += game_object.x_velocity;
        }else {
            if game_object.x_velocity > 0.0{
                game_object.x = x_colider.unwrap().get_left() - game_object.width;
            }else {
                game_object.x = x_colider.unwrap().get_right();
            }
            game_object.x_velocity = -game_object.x_velocity * game_object.bouncyness;
        }

        // y move
        let y_colider = self.check_world_square_collisions(
            game_object.x, 
            game_object.y + game_object.y_velocity, 
            game_object.width, 
            game_object.height);

        if y_colider.is_none(){
            game_object.y += game_object.y_velocity;
        }else {
            if game_object.y_velocity > 0.0{
                game_object.y = y_colider.unwrap().get_top() - game_object.height;
            }else {
                game_object.y = y_colider.unwrap().get_bottom();
            }
            game_object.y_velocity = -game_object.y_velocity * game_object.bouncyness;
        }
    }

    fn check_world_square_collisions(&self, x: f32, y: f32, w: f32, h: f32) -> Option<&WorldTile> {
        let start_x = (x / TILE_SIZE as f32).floor() as i32;
        let end_x = ((x + w) / TILE_SIZE as f32).ceil() as i32;

        let start_y = (y / TILE_SIZE as f32).floor() as i32;
        let end_y = ((y + w) / TILE_SIZE as f32).ceil() as i32;

        for x in start_x..end_x {
            for y in start_y..end_y {
                let tile = self.get_tile(x, y);
                if tile.is_solid() {
                    return Some(tile);
                }
            }
        }

        return None;
    }
}