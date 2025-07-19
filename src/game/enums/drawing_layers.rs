
#[derive(Clone, Copy)]
pub enum DrawingLayer{
    World = 0,
    WorldObjects,
    GameObjects,
    Player,
    Effects,
    UI

}

impl DrawingLayer {
    pub const VALUES: [Self; 6] = [Self::World, Self::WorldObjects, Self::GameObjects, Self::Player, Self::Effects, Self::UI];

    pub fn get_value(&self) -> i32 {
        return self.clone() as i32;
    }
}