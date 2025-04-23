use crate::utils::number_utils::random_integer;

use super::theme_tile::ThemeTile;

pub struct TileCollection{
    tiles: Vec<ThemeTile>,
    combined_weight: i32,
}

impl TileCollection{
    pub fn new(tiles: Vec<ThemeTile>) -> TileCollection{
        
        let mut combined_weight = 0;

        for tile in &tiles {
            combined_weight += tile.get_chance_to_appear();
        }
        
        
        return TileCollection { tiles, combined_weight };
    }

    pub fn get_tile(&self) -> &ThemeTile {
        let chosen_weight = random_integer(0, self.combined_weight);

        let mut current_weight = 0;
        for tile in &self.tiles {
            current_weight += tile.get_chance_to_appear();

            if chosen_weight < current_weight {
                return tile;
            }
        }

        return self.tiles.first().unwrap();
    }
}