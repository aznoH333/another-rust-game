use std::collections::HashMap;

use crate::{engine::{events::{event_manager::EventManager, game_event::GameEvent}, objects::game_object_manager::GameObjectManager, world::{world_generator::WorldGenerator, world_manager::WorldManager}}, game::entities::player::Player, utils::{number_utils::random_integer, space_utils::squares_collide, textures::get_texture_with_index, vec_utils::{pick_random_element_vec, pick_random_index_vec, pick_random_key_map}}};


const WORLD_WIDTH: i32 = 40;
const BORDER_WIDTH: i32 = 8;
const MIN_ROOM_WIDTH: i32 = 3;
const DOOR_SIZE: i32 = 2;

pub struct BasicRoomGenerator{

}

impl BasicRoomGenerator{
    pub fn new() -> BasicRoomGenerator {
        return BasicRoomGenerator {  };
    }


    fn prepare_room(&mut self, world: &mut WorldManager){
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
    }


    fn generate_inner_rooms(&mut self, world: &mut WorldManager){
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
    }


    fn find_rooms(&mut self, world: &mut WorldManager) -> Vec<Room> {
        let mut output = Vec::<Room>::new();

        for x in BORDER_WIDTH..WORLD_WIDTH - BORDER_WIDTH {
            for y in BORDER_WIDTH..WORLD_WIDTH - BORDER_WIDTH {
                // skip is wall
                if world.get_tile(x, y).is_solid(){
                    continue;
                }

                // skip if is already in a room
                let mut is_in_any_room = false;
                for room in &output {
                    if room.is_inside_room(x, y){
                        is_in_any_room = true;
                    }
                }

                if is_in_any_room {
                    continue;
                }

                let start_x = x;
                let start_y = y;

                // get room width
                let mut width = 0;
                loop {
                    if world.get_tile(start_x + width, start_y).is_solid() {
                        break;
                    }
                    width += 1;
                }

                // get room height
                let mut height = 0;
                loop {
                    if world.get_tile(start_x, start_y + height).is_solid() {
                        break;
                    }
                    height += 1;
                }

                // add room
                output.push(Room::new( start_x, start_y, width, height ));
            }
        }

        // find neighbors
        for room_index in 0..output.iter().count() {
            let room = output.get(room_index).unwrap();
            let neighbors = room.find_neighbor_indicies(&output);
            // borrow checker moment 🤡
            output.get_mut(room_index).unwrap().set_neighbors(neighbors);       
        }

        return output;
    }

    // returns starting room index
    fn pick_starting_room(&mut self, rooms: &mut Vec<Room>) -> usize {
        let starting_room_index = pick_random_index_vec(&rooms);
        rooms.get_mut(starting_room_index).expect(format!("invalid room index {}", starting_room_index).as_str()).mark_as_connected();
        return starting_room_index;
    }

    fn create_doors(&mut self, world: &mut WorldManager, starting_room_index: usize, rooms: &mut Vec<Room>) {
        
        // add starting room
        let mut connected_rooms = Vec::<usize>::new();
        connected_rooms.push(starting_room_index);

        loop {

            // collect possible connections
            let mut possible_connections = Vec::<(usize, usize)>::new();

            for room_index in &connected_rooms{
                let room = rooms.get(*room_index).unwrap();
                
                for neighbor_index in &room.neighbors {
                    let neighbor = rooms.get(*neighbor_index).unwrap();

                    if !neighbor.is_connected_to_path {
                        possible_connections.push((*room_index, *neighbor_index));
                    }
                }
            }
    
            // no new connections can be made
            if possible_connections.is_empty() {
                break;
            }


            // pick a random connection and make a door
            let connection = pick_random_element_vec(&possible_connections);

            let origin_room = rooms.get(connection.0).unwrap();
            let target_room = rooms.get(connection.1).unwrap();
            let shared_walls = origin_room.find_shared_walls_with_neighbor(target_room);


            if shared_walls.iter().count() <= DOOR_SIZE as usize {
                for (x, y) in shared_walls {
                    world.make_floor_tile(x, y, "gremlin_0001");
                }
            }else {
                // pick random door location
                let random_door_index = random_integer(0, shared_walls.iter().count() as i32 - DOOR_SIZE);

                for i in random_door_index..random_door_index + DOOR_SIZE {
                    let (x, y) = shared_walls.get(i as usize).unwrap();

                    world.make_floor_tile(*x, *y, "gremlin_0001");
                }

            }

            // mark other as connected
            rooms.get_mut(connection.1).unwrap().mark_as_connected();
            connected_rooms.push(connection.1);

        }

    } 
}

impl WorldGenerator for BasicRoomGenerator{
    fn generate_world(&mut self, world: &mut WorldManager, event_manager: &mut EventManager) {
        
        // prepare world
        world.prepare_world(WORLD_WIDTH, WORLD_WIDTH);

        self.prepare_room(world);
        self.generate_inner_rooms(world);
        let mut rooms = self.find_rooms(world);
        let starting_room_index = self.pick_starting_room(&mut rooms);
        self.create_doors(world, starting_room_index, &mut rooms);
        


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

struct Room {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    is_connected_to_path: bool,
    neighbors: Vec<usize>, 
}

impl Room {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Room {
        return Room {
            x, 
            y, 
            w, 
            h,
            is_connected_to_path: false,
            neighbors: Vec::new(),
        };
    }

    pub fn mark_as_connected(&mut self) {
        self.is_connected_to_path = true;
    }

    pub fn is_connected_to_path(&self) -> bool {
        return self.is_connected_to_path;
    }
    
    pub fn is_inside_room(&self, x: i32, y: i32) -> bool {
        return x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h;
    }

    pub fn find_neighbor_indicies(&self, rooms: &Vec<Room>) -> Vec<usize>{
        let mut output = Vec::<usize>::new();
        
        for (room_index, room) in rooms.iter().enumerate() {
            if self.is_neighbor(room){
                output.push(room_index);
            }
        }

        return output
    }

    pub fn set_neighbors(&mut self, indicies: Vec<usize>){
        self.neighbors = indicies;
    }

    pub fn get_neighbors(&mut self) -> &Vec<usize> {
        return &self.neighbors;
    }

    fn is_neighbor(&self, other: &Room) -> bool {
        return 
            squares_collide(self.x - 2, self.y, 1, self.h, other.x, other.y, other.w, other.h) || // left neighgbor
            squares_collide(self.x + self.w + 2, self.y, 1, self.h, other.x, other.y, other.w, other.h) || // right neighgbor
            squares_collide(self.x, self.y - 2, self.w, 1,other.x, other.y, other.w, other.h) || // top neighbor
            squares_collide(self.x, self.y + self.h + 2, self.w, 1,other.x, other.y, other.w, other.h); // bottom neighbor
    }

    pub fn find_shared_walls_with_neighbor(&self, other: &Room) -> Vec<(i32, i32)> {
        let mut output = Vec::<(i32, i32)>::new();


        let x_overlap_start = self.x.max(other.x);
        let x_overlap_end = (self.x + self.w).min(other.x + other.w);

        let y_overlap_start = self.y.max(other.y);
        let y_overlap_end = (self.y + self.h).min(other.y + other.h);


        if x_overlap_end - x_overlap_start > 0 {
            
            let y = if self.y > other.y { self.y - 1 } else { other.y - 1 };

            for x in x_overlap_start+1..x_overlap_end-1 {
                output.push((x, y));
            }

        }else {

            let x = if self.x > other.x { self.x - 1 } else { other.x - 1 };

            for y in y_overlap_start+1..y_overlap_end-1 {
                output.push((x, y));
            }

        }

        return output;
    }
}