use crate::{engine::world::world_manager::WorldManager, game::world_generators::{data_types::room::Room, temes::{theme_tile::ThemeTile, tile_collection::TileCollection, world_theme::WorldTheme}}, utils::{number_utils::{random_chance, random_integer, random_integer_from_array}, textures::get_texture_with_index}};

pub fn initialize_blue_dungeon_theme() -> WorldTheme{
    
    let mut functions = Vec::<Box<dyn Fn(&mut WorldManager, &Room)>>::new();

    functions.push(Box::new(create_horizontal_table));
    functions.push(Box::new(create_vertical_table));

    
    return WorldTheme::new(
        // wall tiles
        TileCollection::new(Vec::<ThemeTile>::from([
            ThemeTile::new("tiles_0002", 10)
        ])),
        TileCollection::new(Vec::<ThemeTile>::from([
            ThemeTile::new("tiles_0001", 10)
        ])),
        TileCollection::new(Vec::<ThemeTile>::from([
            ThemeTile::new("tiles_0024", 10),
            ThemeTile::new("tiles_0025", 10),
            ThemeTile::new("tiles_0026", 10),
            ThemeTile::new("tiles_0027", 90),

        ])),
        TileCollection::new(Vec::<ThemeTile>::from([
            ThemeTile::new("tiles_0003", 10),
        ])),
        functions,
    );

}

fn make_chair(world:&mut WorldManager, x: i32, y: i32, intended_index: i32){
    let mut index = intended_index;

    if random_chance(20){
        index = random_integer_from_array(&[8, 9]);
    }



    world.make_floor_tile(x, y, &get_texture_with_index("tiles", index));

}


fn create_horizontal_table(world: &mut WorldManager, room: &Room){
    // skip if room too small
    if room.get_width() <= 5 {
        return;
    }

    // pick coordinates to attempt
    let width = random_integer(3, room.get_width() - 3);
    let x = random_integer(1, room.get_width() - 1 - width) + room.get_x();
    let y = random_integer(1, room.get_height() - 2) + room.get_y();

    // check if there are no tiles clipping with the table
    if !world.is_space_empty(x - 1, y - 2, width + 2, 5) {
        return;
    }
    
    // place tiles
    for i in x..x + width {
        // place solids
        world.make_solid_tile(i, y, "tiles_0011");

        // generate chairs
        if random_chance(30){
            make_chair(world, i, y - 1, 6);
        }

        if random_chance(30){
            make_chair(world, i, y + 1, 7);
        }
    }
    // place head & back
    world.make_solid_tile(x, y, "tiles_0010");
    world.make_solid_tile(x + width - 1, y, "tiles_0012");
}


fn create_vertical_table(world: &mut WorldManager, room: &Room){
    // skip if room too small
    if room.get_height() <= 5 {
        return;
    }

    // pick coordinates to attempt
    let height = random_integer(3, room.get_height() - 3);
    let y = random_integer(1, room.get_height() - 1 - height) + room.get_y();
    let x = random_integer(1, room.get_width() - 2) + room.get_x();

    // check if there are no tiles clipping with the table
    if !world.is_space_empty(x - 2, y - 1, 5, height + 2) {
        return;
    }
    
    // place tiles
    for i in y..y + height {
        // place solids
        world.make_solid_tile(x, i, "tiles_0014");

        // generate chairs
        if random_chance(30){
            make_chair(world, x - 1, i, 4);
        }

        if random_chance(30){
            make_chair(world, x + 1, i, 5);
        }
    }
    // place head & back
    world.make_solid_tile(x, y, "tiles_0015");
    world.make_solid_tile(x, y + height - 1, "tiles_0013");
}