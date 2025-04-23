
pub struct Door{
    x: i32,
    y: i32,
}

impl Door{
    pub fn new(x: i32, y: i32) -> Door{
        return Door{ x, y };
    }

    pub fn get_x(&self) -> i32 {
        return self.x;
    }

    pub fn get_y(&self) -> i32 {
        return self.y;
    }

}