use crate::{engine::{events::{event_manager::EventManager, game_event::GameEvent}, objects::game_object_manager::GameObjectManager, world::{world_generator::WorldGenerator, world_manager::WorldManager}}, game::entities::player::Player};


const WORLD_WIDTH: i32 = 40;
const BORDER_WIDTH: i32 = 2;

pub struct BasicRoomGenerator{

}

impl BasicRoomGenerator{
    pub fn new() -> BasicRoomGenerator {
        return BasicRoomGenerator {  };
    }
}

impl WorldGenerator for BasicRoomGenerator{
    fn generate_world(&mut self, world: &mut WorldManager, event_manager: &mut EventManager) {
        
        // prepare world
        world.prepare_world(WORLD_WIDTH, WORLD_WIDTH);

        for i in 0..WORLD_WIDTH{
            for j in 0..BORDER_WIDTH{
                world.set_tile_properties(i, j, "tiles_0001", true);
                world.set_tile_properties(i, WORLD_WIDTH - j - 1, "tiles_0001", true);
                world.set_tile_properties(j,i, "tiles_0001", true);
                world.set_tile_properties(WORLD_WIDTH - j - 1, i, "tiles_0001", true);
            }
        }


        // spawn entities
        event_manager.push_event(GameEvent::SpawnObject { spawn_function: |object_manager: &mut GameObjectManager| {
            object_manager.add_object(Player::new(128.0, 128.0));
        } });
    }
}