
#[derive(Clone, Copy)]
pub enum DrawingLayer{
    World = 0,
    WorldObjects,
    GameObjects,
    Player,
    Effects,
    UI,
    UIFront,

}

impl DrawingLayer {
    pub const VALUES: [Self; 7] = [Self::World, Self::WorldObjects, Self::GameObjects, Self::Player, Self::Effects, Self::UI, Self::UIFront];

    pub fn get_value(&self) -> i32 {
        return self.clone() as i32;
    }
}