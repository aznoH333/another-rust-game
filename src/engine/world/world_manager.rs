use crate::engine::drawing::drawing_manager::DrawingManager;

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
}