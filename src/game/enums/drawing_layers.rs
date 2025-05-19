
#[derive(Clone, Copy)]
pub enum DrawingLayer{
    World = 0,
    WorldObjects,
    GameObjects,
    Player,
}

impl DrawingLayer {
    pub const VALUES: [Self; 4] = [Self::World, Self::WorldObjects, Self::GameObjects, Self::Player];

    pub fn get_value(&self) -> i32 {
        return self.clone() as i32;
    }
}