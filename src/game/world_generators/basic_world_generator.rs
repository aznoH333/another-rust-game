use std::collections::HashMap;

use crate::{engine::{events::{event_manager::EventManager, game_event::GameEvent}, objects::game_object_manager::GameObjectManager, world::{world_generator::WorldGenerator, world_manager::WorldManager}}, game::entities::player::Player, utils::{number_utils::{random_chance, random_integer}, textures::get_texture_with_index, vec_utils::{pick_random_element_vec, pick_random_index_vec, pick_random_key_map}}};

use super::{data_types::{door::Door, door_lock::DoorLockType, point_of_interest::PointOfInterest, room::Room, room_generation_point::RoomGenerationPoint}, temes::world_theme::WorldTheme};


const WORLD_WIDTH: i32 = 40;
const BORDER_WIDTH: i32 = 8;
const MIN_ROOM_WIDTH: i32 = 3;
const DOOR_SIZE: i32 = 1;


pub struct BasicRoomGenerator{
    theme: WorldTheme
}

impl BasicRoomGenerator{
    pub fn new(theme: WorldTheme) -> BasicRoomGenerator {
        return BasicRoomGenerator { theme };
    }


    fn prepare_room(&mut self, world: &mut WorldManager){
        // generate initial square
        for i in 0..WORLD_WIDTH{
            for j in 0..BORDER_WIDTH{
                world.set_tile_properties(i, j, &self.theme.get_wall_tile(), true); // TODO use tiles from theme
                world.set_tile_properties(i, WORLD_WIDTH - j - 1, &self.theme.get_wall_tile(), true);
                world.set_tile_properties(j,i, &self.theme.get_wall_tile(), true);
                world.set_tile_properties(WORLD_WIDTH - j - 1, i, &self.theme.get_wall_tile(), true);
            }
        }

        for i in BORDER_WIDTH..WORLD_WIDTH-BORDER_WIDTH+1{
            world.set_tile_properties(i, BORDER_WIDTH, &self.theme.get_border_tile(), true);
            world.set_tile_properties(i, WORLD_WIDTH - BORDER_WIDTH, &self.theme.get_border_tile(), true);
            world.set_tile_properties(BORDER_WIDTH, i, &self.theme.get_border_tile(), true);
            world.set_tile_properties(WORLD_WIDTH - BORDER_WIDTH, i, &self.theme.get_border_tile(), true);
        }

        // randomize flor sprite
        for x in BORDER_WIDTH+1..WORLD_WIDTH-BORDER_WIDTH{
            for y in BORDER_WIDTH+1..WORLD_WIDTH-BORDER_WIDTH{
                world.make_floor_tile(x, y, &self.theme.get_floor_tile());
            }
        }
    }


    fn generate_inner_rooms(&mut self, world: &mut WorldManager){
        // generate room layout
        // add initial points
        let mut valid_wall_start_points: HashMap<(i32, i32), RoomGenerationPoint> = HashMap::new();
        for i in BORDER_WIDTH + MIN_ROOM_WIDTH + 1 .. WORLD_WIDTH - BORDER_WIDTH - MIN_ROOM_WIDTH {
            valid_wall_start_points.insert((i, BORDER_WIDTH), RoomGenerationPoint::new( false, true, false,  false ));
            valid_wall_start_points.insert((i, WORLD_WIDTH - BORDER_WIDTH), RoomGenerationPoint::new(true, false, false, false));
            valid_wall_start_points.insert((BORDER_WIDTH, i), RoomGenerationPoint::new(false, false, false, true));
            valid_wall_start_points.insert((WORLD_WIDTH - BORDER_WIDTH, i), RoomGenerationPoint::new(false, false, true, false));
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
                iteration_count += 1;
                world.make_solid_tile(x, y, &self.theme.get_border_tile());
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
                
                for neighbor_index in room.get_neighbors() {
                    let neighbor = rooms.get(*neighbor_index).unwrap();

                    if !neighbor.is_connected_to_path() {
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
                    rooms.get_mut(connection.0).unwrap().add_door(Door::new(x, y));
                    rooms.get_mut(connection.1).unwrap().add_door(Door::new(x, y));

                    world.make_floor_tile(x, y, &self.theme.get_door_tile());
                }
            }else {
                // pick random door location
                let random_door_index = random_integer(0, shared_walls.iter().count() as i32 - DOOR_SIZE);

                for i in random_door_index..random_door_index + DOOR_SIZE {
                    let (x, y) = shared_walls.get(i as usize).unwrap();

                    world.make_floor_tile(*x, *y, &self.theme.get_door_tile());

                    rooms.get_mut(connection.0).unwrap().add_door(Door::new(*x, *y));
                    rooms.get_mut(connection.1).unwrap().add_door(Door::new(*x, *y));
                }

            }

            // mark other as connected
            rooms.get_mut(connection.1).unwrap().mark_as_connected();
            connected_rooms.push(connection.1);

        }

    } 


    fn assign_special_rooms(&mut self, starting_room_index: usize, rooms: &mut Vec<Room>) {
        let mut special_room_candidates = Vec::<usize>::new();
        // find special room candidates
        for (index, room) in rooms.iter().enumerate() {
            if index == starting_room_index {
                continue;
            }    

            if room.get_neighbors().iter().count() == 1 && room.get_special() == &PointOfInterest::None {
                special_room_candidates.push(index);
            }
        }
        

        // assign specials
        rooms.get_mut(starting_room_index).unwrap().make_room_special(PointOfInterest::PlayerSpawn);
        
        
        let mut shop_index = -1;
        let mut is_shop_locked = false;
        let mut button_index = -1;
        let mut button_lock_shop = false;

        // shop room
        if special_room_candidates.iter().count() > 1 && random_chance(40) {
            // choose room at random
            let chosen_room_index = pick_random_index_vec(&special_room_candidates);

            shop_index = *special_room_candidates.get(chosen_room_index).unwrap() as i32;

            let shop_room = rooms.get_mut(shop_index as usize).unwrap();

            shop_room.make_room_special(PointOfInterest::Shop);


            if random_chance(33) {
                shop_room.lock_room(DoorLockType::Key);
                is_shop_locked = true;
            }

            special_room_candidates.remove(chosen_room_index);
        }


        // button room
        if special_room_candidates.iter().count() > 1 && random_chance(70){
            let chosen_room_index = pick_random_index_vec(&special_room_candidates);
            button_index = *special_room_candidates.get(chosen_room_index).unwrap() as i32;
            special_room_candidates.remove(chosen_room_index);
            rooms.get_mut(button_index as usize).unwrap().make_room_special(PointOfInterest::Button);

            if random_chance(50) && !is_shop_locked {
                rooms.get_mut(shop_index as usize).unwrap().lock_room(DoorLockType::Button);
            }else {
                button_lock_shop = true;
            }
        }

        // exit room
        let exit_room_index = pick_random_element_vec(&special_room_candidates);
        let exit_room = rooms.get_mut(*exit_room_index).unwrap();
        exit_room.make_room_special(PointOfInterest::Exit);
        if button_lock_shop {
            exit_room.lock_room(DoorLockType::Button);
        }
        
        // assign common specials
        for (index, room) in rooms.iter_mut().enumerate() {
            if index != *exit_room_index && index != starting_room_index && index as i32 != shop_index && index as i32 != button_index {
                if random_chance(33){
                    room.make_room_special(PointOfInterest::BigFight);
                }else if random_chance(50){
                    room.make_room_special(PointOfInterest::Treasure);
                }
            }
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
        // self.assign_special_rooms(starting_room_index, &mut rooms);


        // spawn entities
        event_manager.push_event(GameEvent::SpawnObject { spawn_function: |object_manager: &mut GameObjectManager| {
            object_manager.add_object(Player::new(128.0, 128.0));
        } });
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

