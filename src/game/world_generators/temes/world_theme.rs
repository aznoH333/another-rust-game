use super::tile_collection::TileCollection;

pub struct WorldTheme{
    wall_tiles: TileCollection,
    border_tiles: TileCollection,
    floor_tiles: TileCollection,
    door_tiles: TileCollection,
}


impl WorldTheme{
    pub fn new(
        wall_tiles: TileCollection,
        border_tiles: TileCollection,
        floor_tiles: TileCollection,
        door_tiles: TileCollection,
    ) -> WorldTheme {

        return WorldTheme { 
            wall_tiles, 
            border_tiles, 
            floor_tiles,
            door_tiles
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
}