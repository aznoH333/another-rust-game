
#[derive(Clone, Copy)]
pub enum DrawingLayer{
    World = 0,
    WorldObjects,
    Player,
}

impl DrawingLayer {
    pub const VALUES: [Self; 3] = [Self::World, Self::WorldObjects, Self::Player];

    pub fn get_value(&self) -> i32 {
        return self.clone() as i32;
    }
}