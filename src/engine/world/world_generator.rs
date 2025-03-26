use super::world_manager::WorldManager;

pub trait WorldGenerator{
    fn generate_world(&mut self, world: &mut WorldManager);
}