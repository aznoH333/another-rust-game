use crate::{engine::{drawing::drawing_manager::DrawingManager, events::event_manager::{EventManager}, objects::{game_object_core::GameObjectCore}}, utils::space_utils::SpaceUtils};

use super::{world_constants::TILE_SIZE, world_generator::WorldGenerator, world_tile::WorldTile};

pub struct WorldManager{
    world: Vec<Vec<WorldTile>>,
    is_world_prepared: bool,
}

impl WorldManager{
    pub fn new(generator: &mut dyn WorldGenerator, event_manager: &mut EventManager) -> WorldManager {
        let mut this = WorldManager{
            world: Vec::new(),
            is_world_prepared: false,
        };

        generator.generate_world(&mut this, event_manager);
        return this;
    }


    pub fn prepare_world(&mut self, width: i32, height: i32){
        if self.is_world_prepared {
            panic!("ERROR : called prepare world after it was already initialized")
        }

        for x in 0..width{
            if self.world.get(x as usize).is_none() {
                self.world.insert(x as usize, Vec::new());
            }
            
            for y in 0..height{
                self.world.get_mut(x as usize).unwrap().insert(y as usize, 
                    WorldTile::new(false, "tiles_0005", x * TILE_SIZE, y * TILE_SIZE)
                );
            }
        }

        self.is_world_prepared = true;
    }

    fn get_tile_mut(&mut self, x: i32, y: i32) -> &mut WorldTile{
        return self.world.get_mut(x as usize).unwrap().get_mut(y as usize).unwrap();
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &WorldTile{
        return self.world.get(x as usize).unwrap().get(y as usize).unwrap();
    }

    fn is_tile_solid(&self, x: i32, y: i32) -> bool {
        return self.get_tile(x, y).is_solid()
    }

    pub fn set_tile_properties(&mut self, x: i32, y: i32, texture: &str, is_solid: bool){
        let tile = self.get_tile_mut(x, y);
        tile.set_texture(texture);
        tile.set_solid(is_solid);
    }

    pub fn make_solid_tile(&mut self, x: i32, y: i32, texture: &str){
        self.set_tile_properties(x, y, texture,true);
    }

    pub fn make_floor_tile(&mut self, x: i32, y: i32, texture: &str){
        self.set_tile_properties(x, y, texture, false);
    }


    pub fn draw_world(&mut self, drawing: &mut DrawingManager){
        for v in &self.world {
            for tile in v{
                tile.draw(drawing);
            }
        }
    }

    pub fn move_in_world(&self, game_object: &mut GameObjectCore, delta: f32) -> bool {
        let mut collided = false;
        
        // NOTE : this is 🤡 code. 
        // afaik bool ? expresion : expresion doesn't exist in rust
        // this for some reason executes collide_with_terrain anyway
        // so the app will crash if an object goes out of bounds even if its a ghost
        // x move
        let x_colider = game_object.collide_with_terrain.then_some(self.check_world_square_collisions(
            game_object.left() + game_object.get_x_velocity(), 
            game_object.top(), 
            game_object.get_width(), 
            game_object.get_height())).unwrap_or(None);

        if x_colider.is_none(){
            game_object.set_x(game_object.get_x() + game_object.get_x_velocity())
        }else {
            if game_object.get_x() < x_colider.unwrap().get_center_x(){
                game_object.set_x(x_colider.unwrap().get_left() - (game_object.get_width() / 2.0));
            }else {
                game_object.set_x(x_colider.unwrap().get_right() + (game_object.get_width() / 2.0));
            }
            game_object.x_velocity = -game_object.x_velocity * game_object.bounciness;
            collided = true;
        }

        // y move
        let y_colider = game_object.collide_with_terrain.then_some(self.check_world_square_collisions(
            game_object.left(), 
            game_object.top() + game_object.get_y_velocity(), 
            game_object.get_width(), 
            game_object.get_height())).unwrap_or(None);

        if y_colider.is_none(){
            game_object.set_y(game_object.get_y() + game_object.get_y_velocity());
        }else {
            if game_object.get_y() < y_colider.unwrap().get_center_y() {
                game_object.set_y(y_colider.unwrap().get_top() - (game_object.get_height() / 2.0));
            }else {
                game_object.set_y(y_colider.unwrap().get_bottom() + (game_object.get_height() / 2.0));

            }
            game_object.y_velocity = -game_object.y_velocity * game_object.bounciness;
            collided = true;
        }

        return collided;
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

    pub fn is_space_empty(&self, x: i32, y: i32, w: i32, h: i32) -> bool {
        for a in x..x+w {
            for b in y..y+h{
                if self.is_tile_solid(a, b){
                    return false;
                }
            }
        }

        return true;
    }


    pub fn is_tile_empty(&self, x: i32, y: i32) -> bool {
        return !self.is_tile_solid(x, y);
    }

    pub fn has_line_of_sight(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
        let distance = SpaceUtils::pythagoras(x1, y1, x2, y2);
        
        let x_speed = (x2 - x1) / distance;
        let y_speed = (y2 - y1) / distance;

        let mut current_x = x1;
        let mut current_y = y1;

        for _ in 0..(distance.ceil() as i32) {
            current_x += x_speed;
            current_y += y_speed;

            // check if solid
            let world_x = SpaceUtils::game_units_to_world_units(current_x);
            let world_y = SpaceUtils::game_units_to_world_units(current_y);

            if self.is_tile_solid(world_x, world_y) {
                return false;
            }
        }

        return true;
    }
}