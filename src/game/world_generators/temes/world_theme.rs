use std::ops::Range;

use crate::{engine::world::world_manager::WorldManager, game::world_generators::data_types::room::Room, utils::{number_utils::NumberUtils, vec_utils::VecUtils}};

use super::tile_collection::TileCollection;

pub struct WorldTheme{
    wall_tiles: TileCollection,
    border_tiles: TileCollection,
    floor_tiles: TileCollection,
    door_tiles: TileCollection,
    decorate_functions: Vec::<Box<dyn Fn(&mut WorldManager, &Room) -> bool>>,
    tiles_per_decoration: i32,
    number_of_walls: Range<i32>,
}


impl WorldTheme{
    pub fn new(
        wall_tiles: TileCollection,
        border_tiles: TileCollection,
        floor_tiles: TileCollection,
        door_tiles: TileCollection,
        decorate_functions: Vec::<Box<dyn Fn(&mut WorldManager, &Room) -> bool>>,
        decorations_per_room: i32,
        number_of_walls: Range<i32>,
    ) -> WorldTheme {

        return WorldTheme { 
            wall_tiles, 
            border_tiles, 
            floor_tiles,
            door_tiles,
            decorate_functions,
            tiles_per_decoration: decorations_per_room,
            number_of_walls,
        };

    }

    pub fn get_wall_tile(&self) -> String {
        return self.wall_tiles.get_tile().get_texture_name().to_owned();
    }

    pub fn get_border_tile(&self) -> String {
        return self.border_tiles.get_tile().get_texture_name().to_owned();
    }

    pub fn get_floor_tile(&self) -> String {
        return self.floor_tiles.get_tile().get_texture_name().to_owned();
    }

    pub fn get_door_tile(&self) -> String {
        return self.door_tiles.get_tile().get_texture_name().to_owned();
    }

    pub fn pick_random_decorator(&self) -> &Box<dyn Fn(&mut WorldManager, &Room) -> bool> {
        return VecUtils::pick_random_element_vec(&self.decorate_functions);
    }

    pub fn get_number_of_tiles_per_decoration(&self) -> i32 {
        return self.tiles_per_decoration;
    }

    pub fn get_number_of_walls(&self) -> i32 {
        return NumberUtils::random_integer(self.number_of_walls.start, self.number_of_walls.end);
    }
}