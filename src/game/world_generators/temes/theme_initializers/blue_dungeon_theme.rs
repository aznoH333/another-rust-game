
use crate::{engine::world::world_manager::WorldManager, game::world_generators::{data_types::room::Room, temes::{theme_tile::ThemeTile, tile_collection::TileCollection, world_theme::WorldTheme}}, utils::{number_utils::NumberUtils, textures::TextureUtils}};

pub fn initialize_blue_dungeon_theme() -> WorldTheme{
    
    let mut functions = Vec::<Box<dyn Fn(&mut WorldManager, &Room) -> bool>>::new();

    functions.push(Box::new(decorate_horizontal_table));
    functions.push(Box::new(decorate_vertical_table));
    functions.push(Box::new(decorate_horizontal_crates));
    functions.push(Box::new(decorate_vertical_crates));
    functions.push(Box::new(decorate_bone_pile));

    
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
        20,
        5..7,
    );

}

fn make_chair(world:&mut WorldManager, x: i32, y: i32, intended_index: i32){
    let mut index = intended_index;

    if NumberUtils::random_chance(20){
        index = NumberUtils::random_integer_from_array(&[8, 9]);
    }



    world.make_floor_tile(x, y, &TextureUtils::get_texture_with_index("tiles", index));

}


fn decorate_horizontal_table(world: &mut WorldManager, room: &Room) -> bool{
    // skip if room too small
    if room.get_width() <= 5 {
        return false;
    }

    // pick coordinates to attempt
    let width = NumberUtils::random_integer(3, room.get_width() - 3);
    let x = NumberUtils::random_integer(1, room.get_width() - 1 - width) + room.get_x();
    let y = NumberUtils::random_integer(1, room.get_height() - 2) + room.get_y();

    // check if there are no tiles clipping with the table
    if !world.is_space_empty(x - 1, y - 2, width + 2, 5) {
        return false;
    }
    
    // place tiles
    for i in x..x + width {
        // place solids
        world.make_solid_tile(i, y, "tiles_0011");

        // generate chairs
        if NumberUtils::random_chance(30){
            make_chair(world, i, y - 1, 6);
        }

        if NumberUtils::random_chance(30){
            make_chair(world, i, y + 1, 7);
        }
    }
    // place head & back
    world.make_solid_tile(x, y, "tiles_0010");
    world.make_solid_tile(x + width - 1, y, "tiles_0012");

    return true;
}


fn decorate_vertical_table(world: &mut WorldManager, room: &Room) -> bool{
    // skip if room too small
    if room.get_height() <= 5 {
        return false;
    }

    // pick coordinates to attempt
    let height = NumberUtils::random_integer(3, room.get_height() - 3);
    let y = NumberUtils::random_integer(1, room.get_height() - 1 - height) + room.get_y();
    let x = NumberUtils::random_integer(1, room.get_width() - 2) + room.get_x();

    // check if there are no tiles clipping with the table
    if !world.is_space_empty(x - 2, y - 1, 5, height + 2) {
        return false;
    }
    
    // place tiles
    for i in y..y + height {
        // place solids
        world.make_solid_tile(x, i, "tiles_0014");

        // generate chairs
        if NumberUtils::random_chance(30){
            make_chair(world, x - 1, i, 4);
        }

        if NumberUtils::random_chance(30){
            make_chair(world, x + 1, i, 5);
        }
    }
    // place head & back
    world.make_solid_tile(x, y, "tiles_0015");
    world.make_solid_tile(x, y + height - 1, "tiles_0013");

    return true;
}

const CRATE_SPREAD_DISTANCE: i32 = 7;
fn decorate_horizontal_crates(world: &mut WorldManager, room: &Room) -> bool{
    let x = NumberUtils::random_integer(0, room.get_width()) + room.get_x();
    let y = if NumberUtils::random_chance(50) { room.get_y() } else { room.get_y() + room.get_height() - 1 };

    let mut placed_tiles = 0;
    for i in (x - CRATE_SPREAD_DISTANCE).max(room.get_left()) .. (x + CRATE_SPREAD_DISTANCE).min(room.get_right()) {
        if room.calculate_distance_to_door(i, y) > 5.0 && NumberUtils::random_chance((CRATE_SPREAD_DISTANCE - (i-x).abs()).max(1) * 10) { // TODO : possibly unknown doors?

            if NumberUtils::random_chance(60) {
                // place crate
                world.make_solid_tile(i, y, "tiles_0016");
            }else {
                // place barrel
                world.make_solid_tile(i, y, "tiles_0017");

            }

            placed_tiles += 1;

        }
    }

    return placed_tiles != 0;
}


fn decorate_vertical_crates(world: &mut WorldManager, room: &Room) -> bool {
    let x = if NumberUtils::random_chance(50) { room.get_x() } else { room.get_x() + room.get_width() - 1 };
    let y = NumberUtils::random_integer(0, room.get_height()) + room.get_y();

    let mut placed_tiles = 0;

    for i in (y - CRATE_SPREAD_DISTANCE).max(room.get_top()) .. (y + CRATE_SPREAD_DISTANCE).min(room.get_bottom()) {
        if room.calculate_distance_to_door(x, i) > 2.0 && NumberUtils::random_chance((CRATE_SPREAD_DISTANCE - (i-y).abs()).max(1) * 10) {

            if NumberUtils::random_chance(60) {
                // place crate
                world.make_solid_tile(x, i, "tiles_0016");
            }else {
                // place barrel
                world.make_solid_tile(x, i, "tiles_0017");

            }
            placed_tiles += 1;

        }
    }

    return placed_tiles != 0;
}

const BONE_MAX_OFFSET: i32 = 2;
fn decorate_bone_pile(world: &mut WorldManager, room: &Room) -> bool {
    let x = NumberUtils::random_integer(room.get_x(), room.get_x() + room.get_width());
    let y = NumberUtils::random_integer(room.get_y(), room.get_y() + room.get_height());

    for _ in 0..6 {
        let bone_x = NumberUtils::random_integer(x - BONE_MAX_OFFSET, x + BONE_MAX_OFFSET);
        let bone_y = NumberUtils::random_integer(y - BONE_MAX_OFFSET, y + BONE_MAX_OFFSET);

        if room.is_inside_room(bone_x, bone_y) && world.is_tile_empty(bone_x, bone_y){
            world.make_floor_tile(bone_x, bone_y, &TextureUtils::get_texture_with_index("tiles", NumberUtils::random_integer(33, 37)));
        }
    }

    return true;
}