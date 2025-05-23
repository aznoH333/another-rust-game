use crate::{engine::world::{world_constants::TILE_SIZE, world_manager::WorldManager}, utils::{space_utils::SpaceUtils, vec_utils::VecUtils}};

use super::{door::Door, door_lock::DoorLockType, point_of_interest::PointOfInterest};

/**
 An object representing a room inside a dungeon.
 This object doesn't store the content's of a room, It just holds information about the placement of the room
 and some other relevant information that is usefull when constructing a room.
 */
pub struct Room {
    x: i32,
    y: i32,
    // width of a room
    w: i32,
    // height of a room
    h: i32,
    /// a variable used only during dungeon generation
    is_connected_to_path: bool,
    neighbors: Vec<usize>, 
    doors: Vec<Door>,
    door_lock: DoorLockType,
    /// determines the purpose of a room
    special_type: PointOfInterest,
    /// an array with a distance to closest door calculated for each non solid tile (x, y, distance to nearest door)
    tile_distance_weights: Vec<(f32, f32, f32)>, 
    /// the smallest distance between a door and any non solid tile
    min_dist_to_door: f32, 
    /// the largest distance between a door and any non solid tile
    max_dist_to_door: f32, 
}

impl Room {
    /**
     Creates a new room object with specified x/y and width/height.
     Positions are stored in tiles not pixels. \
     **Parameters:**
     * x - the x position of a room (left side)
     * y - the y position of a room (top side)
     * w - the width of a room
     * h - the height of a room
     */
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Room {
        return Room {
            x, 
            y, 
            w, 
            h,
            is_connected_to_path: false,
            neighbors: Vec::new(),
            doors: Vec::new(),
            door_lock: DoorLockType::None,
            special_type: PointOfInterest::None,
            tile_distance_weights: Vec::new(),
            min_dist_to_door: 999.9,
            max_dist_to_door: -1.0,
        };
    }

    pub fn mark_as_connected(&mut self) {
        self.is_connected_to_path = true;
    }

    pub fn is_connected_to_path(&self) -> bool {
        return self.is_connected_to_path;
    }
    /**
     Checks if a tile is positioned inside the room \
     **Parameters:**
     * x - the x position of the tile
     * y - the y position of the tile
     \
     **Returns:** a boolean indicating if a tile is inside a room

     */
    pub fn is_inside_room(&self, x: i32, y: i32) -> bool {
        return x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h;
    }

    /**
     Looks for neighboring rooms in a given rooms array.
     Returns the result as an array due to some dumb borrow checker fuckery. \
     **Parameters:**
     * rooms - an array containing all rooms inside a dungeon
     \
     **Returns:** An array with the indicies of the neighboring rooms. Indicies point to the array passed as parameter.
     */
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

    pub fn get_neighbors(&self) -> &Vec<usize> {
        return &self.neighbors;
    }

    fn is_neighbor(&self, other: &Room) -> bool {
        return 
            (SpaceUtils::squares_collide(self.x - 2, self.y + 1, 1, self.h - 2, other.x, other.y, other.w, other.h) || // left neighgbor
            SpaceUtils::squares_collide(self.x + self.w + 2, self.y + 1, 1, self.h - 2, other.x, other.y, other.w, other.h) || // right neighgbor
            SpaceUtils::squares_collide(self.x + 1, self.y - 2, self.w - 2, 1,other.x, other.y, other.w, other.h) || // top neighbor
            SpaceUtils::squares_collide(self.x + 1, self.y + self.h + 2, self.w - 2, 1,other.x, other.y, other.w, other.h)) // bottom neighbor
            && !self.find_shared_walls_with_neighbor(other).is_empty(); // have a shared wall
    }

    pub fn get_doors(&self) -> &Vec<Door> {
        return &self.doors;
    }
    /**
     Returns a list of shared walls with a given room. These are tiles where a door could generate. \
     **Parameters:**
     * other - The neighboring room.
     \
     **Returns:** A list of shared walls (returned as an (x, y) vec)
     */
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

    pub fn add_door(&mut self, door: Door) {
        self.doors.push(door);
    }

    pub fn make_room_special(&mut self, special: PointOfInterest){
        self.special_type = special;
    }

    pub fn lock_room(&mut self, lock: DoorLockType){
        self.door_lock = lock;
    }

    pub fn get_special(&self) -> &PointOfInterest {
        return &self.special_type;
    }

    pub fn get_x(&self) -> i32 {
        return self.x;
    }

    pub fn get_y(&self) -> i32 {
        return self.y;
    }

    pub fn get_width(&self) -> i32 {
        return self.w;
    }

    pub fn get_height(&self) -> i32 {
        return self.h;
    }

    pub fn get_surface(&self) -> i32 {
        return self.w * self.h;
    }

    pub fn get_left(&self) -> i32 {
        return self.x;
    }

    pub fn get_top(&self) -> i32 {
        return self.y;
    }

    pub fn get_right(&self) -> i32 {
        return self.x + self.w;
    }

    pub fn get_bottom(&self) -> i32 {
        return self.y + self.h;
    }

    pub fn calculate_distance_to_door(&self, x: i32, y: i32) -> f32 {
        let mut output: f32 = 9999.0;

        for door in &self.doors {
            let distance_to_door = SpaceUtils::pythagoras(door.get_x() as f32, door.get_y() as f32, x as f32, y as f32);
            output = output.min(distance_to_door);
        }

        return output;
    }

    /**
    Iterates through every tile in the room and calculatest its distance to the nearest door.
    Saves the results to self.tile_distance_weights \
    **Parameters:**
    * world_manager - An instance of WorldManager
     */
    pub fn calculare_distances_to_doors(&mut self, world_manager: &WorldManager){
        for x in self.get_left()..self.get_right()+1 {
            for y in self.get_top()..self.get_bottom()+1 {
                if !world_manager.is_tile_empty(x, y) {
                    continue;
                }

                let dist = self.calculate_distance_to_door(x, y);
                self.min_dist_to_door = self.min_dist_to_door.min(dist);
                self.max_dist_to_door = self.max_dist_to_door.max(dist);
                self.tile_distance_weights.push(((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32, dist));
            }
        }
    }


    pub fn pick_random_empty_spot_in_room(&self) -> (f32, f32) {
        return self.pick_random_empty_spot_with_distance( 0.0);
    }
    

    pub fn pick_random_empty_spot_with_distance(&self, desired_dist: f32) -> (f32, f32) {
        if self.tile_distance_weights.is_empty() {
            panic!("Shit just hit the fan! Room generated with no valid spawn points!!!!");
        }else {
            let acceptable_distance = (self.max_dist_to_door - self.min_dist_to_door) * desired_dist;
            let filtered: Vec::<(f32, f32)> = self.tile_distance_weights.iter().filter(|it|{
                return it.2 >= acceptable_distance
            }).map(|it|{return (it.0, it.1)}).collect();

            return VecUtils::pick_random_element_vec(&filtered).to_owned();
        }

    }

}