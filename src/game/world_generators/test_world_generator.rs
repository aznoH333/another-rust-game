use crate::engine::world::world_generator::WorldGenerator;

pub struct TestWorldGenerator{

}

impl TestWorldGenerator{
    pub fn new() -> TestWorldGenerator {
        return TestWorldGenerator {  };
    }
}

impl WorldGenerator for TestWorldGenerator{
    fn generate_world(&mut self, world: &mut crate::engine::world::world_manager::WorldManager) {
        world.prepare_world(100, 100);

        for i in 3..5{
            world.set_tile_properties(i, 3, "tiles_0001", true);
            world.set_tile_properties(3, i, "tiles_0001", true);
        }
    }
}
