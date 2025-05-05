use crate::{engine::{events::event_manager::{self, EventManager}, world::world_manager::{self, WorldManager}}, utils::space_utils::{pythagoras, squares_collide}};

use super::{door::Door, door_lock::DoorLockType, point_of_interest::PointOfInterest};

pub struct Room {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    is_connected_to_path: bool,
    neighbors: Vec<usize>, 
    doors: Vec<Door>,
    door_lock: DoorLockType,
    special_type: PointOfInterest,
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
            doors: Vec::new(),
            door_lock: DoorLockType::None,
            special_type: PointOfInterest::None,
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

    pub fn get_neighbors(&self) -> &Vec<usize> {
        return &self.neighbors;
    }

    fn is_neighbor(&self, other: &Room) -> bool {
        return 
            (squares_collide(self.x - 2, self.y + 1, 1, self.h - 2, other.x, other.y, other.w, other.h) || // left neighgbor
            squares_collide(self.x + self.w + 2, self.y + 1, 1, self.h - 2, other.x, other.y, other.w, other.h) || // right neighgbor
            squares_collide(self.x + 1, self.y - 2, self.w - 2, 1,other.x, other.y, other.w, other.h) || // top neighbor
            squares_collide(self.x + 1, self.y + self.h + 2, self.w - 2, 1,other.x, other.y, other.w, other.h)) // bottom neighbor
            && !self.find_shared_walls_with_neighbor(other).is_empty(); // have a shared wall
    }

    pub fn get_doors(&self) -> &Vec<Door> {
        return &self.doors;
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

    pub fn add_door(&mut self, door: Door) {
        self.doors.push(door);
    }

    pub fn generate_contents(&self, world_manager: WorldManager){

    }

    pub fn spawn_contents(&self, event_manager: &mut EventManager){
        todo!("this")
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
            let distance_to_door = pythagoras(door.get_x() as f32, door.get_y() as f32, x as f32, y as f32);
            output = output.min(distance_to_door);
        }

        return output;
    }
}