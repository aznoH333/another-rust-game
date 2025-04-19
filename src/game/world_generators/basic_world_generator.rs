use std::collections::HashMap;

use crate::{engine::{events::{event_manager::EventManager, game_event::GameEvent}, objects::game_object_manager::GameObjectManager, world::{world_generator::WorldGenerator, world_manager::WorldManager}}, game::entities::player::Player, utils::{number_utils::random_integer, textures::get_texture_with_index, vec_utils::{pick_random_element_vec, pick_random_key_map}}};


const WORLD_WIDTH: i32 = 40;
const BORDER_WIDTH: i32 = 8;
const MIN_ROOM_WIDTH: i32 = 3;

pub struct BasicRoomGenerator{

}

impl BasicRoomGenerator{
    pub fn new() -> BasicRoomGenerator {
        return BasicRoomGenerator {  };
    }
}

impl WorldGenerator for BasicRoomGenerator{
    fn generate_world(&mut self, world: &mut WorldManager, event_manager: &mut EventManager) {
        
        // prepare world
        world.prepare_world(WORLD_WIDTH, WORLD_WIDTH);

        // generate initial square
        for i in 0..WORLD_WIDTH{
            for j in 0..BORDER_WIDTH{
                world.set_tile_properties(i, j, "tiles_0002", true);
                world.set_tile_properties(i, WORLD_WIDTH - j - 1, "tiles_0002", true);
                world.set_tile_properties(j,i, "tiles_0002", true);
                world.set_tile_properties(WORLD_WIDTH - j - 1, i, "tiles_0002", true);
            }
        }

        for i in BORDER_WIDTH..WORLD_WIDTH-BORDER_WIDTH+1{
            world.set_tile_properties(i, BORDER_WIDTH, "tiles_0001", true);
            world.set_tile_properties(i, WORLD_WIDTH - BORDER_WIDTH, "tiles_0001", true);
            world.set_tile_properties(BORDER_WIDTH, i, "tiles_0001", true);
            world.set_tile_properties(WORLD_WIDTH - BORDER_WIDTH, i, "tiles_0001", true);
        }

        // randomize flor sprite
        for x in BORDER_WIDTH+1..WORLD_WIDTH-BORDER_WIDTH-1{
            for y in BORDER_WIDTH+1..WORLD_WIDTH-BORDER_WIDTH-1{
                world.make_floor_tile(x, y, &get_texture_with_index("tiles", random_integer(3, 6)));
            }
        }



        // generate room layout
        // add initial points
        let mut valid_wall_start_points: HashMap<(i32, i32), RoomGenerationPoint> = HashMap::new();
        for i in BORDER_WIDTH + MIN_ROOM_WIDTH + 1 .. WORLD_WIDTH - BORDER_WIDTH - MIN_ROOM_WIDTH {
            valid_wall_start_points.insert((i, BORDER_WIDTH), RoomGenerationPoint { allow_up: false, allow_down: true, allow_left: false, allow_right: false });
            valid_wall_start_points.insert((i, WORLD_WIDTH - BORDER_WIDTH), RoomGenerationPoint { allow_up: true, allow_down: false, allow_left: false, allow_right: false });
            valid_wall_start_points.insert((BORDER_WIDTH, i), RoomGenerationPoint { allow_up: false, allow_down: false, allow_left: false, allow_right: true });
            valid_wall_start_points.insert((WORLD_WIDTH - BORDER_WIDTH, i), RoomGenerationPoint { allow_up: false, allow_down: false, allow_left: true, allow_right: false });
        }

        let wall_number = random_integer(4, 7);

        for _ in 0..wall_number{
            // pick point
            let random_point = pick_random_key_map(&valid_wall_start_points).to_owned();
            // pick dirrection
            let point_attributes = valid_wall_start_points.get(&random_point).unwrap();
            let (x_dir, y_dir) = point_attributes.get_valid_direction();
            let (mut x,mut y) = random_point;
            eliminate_wall_start_points_around_point(&random_point, &mut valid_wall_start_points);

            let mut iteration_count = 0;
            // place walls unitl an end is reached
            loop {
                x += x_dir;
                y += y_dir;

                if world.get_tile(x, y).is_solid(){
                    break;
                }

                if iteration_count > MIN_ROOM_WIDTH {
                    valid_wall_start_points.insert((x, y), RoomGenerationPoint::from_direction((x_dir, y_dir)));
                }

                world.make_solid_tile(x, y, "tiles_0001");
            }
            eliminate_wall_start_points_around_point(&(x, y), &mut valid_wall_start_points);
        }


        // spawn entities
        event_manager.push_event(GameEvent::SpawnObject { spawn_function: |object_manager: &mut GameObjectManager| {
            object_manager.add_object(Player::new(128.0, 128.0));
        } });
    }
}


struct RoomGenerationPoint{
    allow_up: bool,
    allow_down: bool,
    allow_left: bool,
    allow_right: bool,
}

impl RoomGenerationPoint {
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


fn eliminate_wall_start_points_around_point(point: &(i32, i32), start_points: &mut HashMap<(i32, i32), RoomGenerationPoint>) {
    for x in point.0 - MIN_ROOM_WIDTH .. point.0 + MIN_ROOM_WIDTH + 1 {
        for y in point.1 - MIN_ROOM_WIDTH .. point.1 + MIN_ROOM_WIDTH + 1 {
            if start_points.contains_key(&(x, y)) {
                start_points.remove(&(x, y)).unwrap();
            }
        }
    }
}