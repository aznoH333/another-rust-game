use crate::utils::space_utils::squares_collide;

pub struct Room {
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

    pub fn get_neighbors(&self) -> &Vec<usize> {
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