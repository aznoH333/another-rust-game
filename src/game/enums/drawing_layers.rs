
#[derive(Clone, Copy)]
pub enum DrawingLayer{
    WORLD = 0,
    PLAYER,
}

impl DrawingLayer {
    pub const VALUES: [Self; 2] = [Self::WORLD, Self::PLAYER];

    pub fn get_value(&self) -> i32 {
        return self.clone() as i32;
    }
}