use crate::{engine::{events::{event_manager::EventManager, game_event::GameEvent}, objects::game_object_manager::GameObjectManager, world::{world_generator::WorldGenerator, world_manager::WorldManager}}, game::entities::player::Player, utils::{number_utils::random_integer, textures::get_texture_with_index}};


const WORLD_WIDTH: i32 = 40;
const BORDER_WIDTH: i32 = 8;

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

        // generate initial square
        for i in 0..WORLD_WIDTH{
            for j in 0..BORDER_WIDTH{
                world.set_tile_properties(i, j, "tiles_0002", true);
                world.set_tile_properties(i, WORLD_WIDTH - j - 1, "tiles_0002", true);
                world.set_tile_properties(j,i, "tiles_0002", true);
                world.set_tile_properties(WORLD_WIDTH - j - 1, i, "tiles_0002", true);
            }
        }

        for i in BORDER_WIDTH..WORLD_WIDTH-BORDER_WIDTH+1{
            world.set_tile_properties(i, BORDER_WIDTH, "tiles_0001", true);
            world.set_tile_properties(i, WORLD_WIDTH - BORDER_WIDTH, "tiles_0001", true);
            world.set_tile_properties(BORDER_WIDTH, i, "tiles_0001", true);
            world.set_tile_properties(WORLD_WIDTH - BORDER_WIDTH, i, "tiles_0001", true);
        }

        // randomize flor sprite
        for x in BORDER_WIDTH+1..WORLD_WIDTH-BORDER_WIDTH-1{
            for y in BORDER_WIDTH+1..WORLD_WIDTH-BORDER_WIDTH-1{
                world.make_floor_tile(x, y, &get_texture_with_index("tiles", random_integer(3, 6)));
            }
        }


        


        // spawn entities
        event_manager.push_event(GameEvent::SpawnObject { spawn_function: |object_manager: &mut GameObjectManager| {
            object_manager.add_object(Player::new(128.0, 128.0));
        } });
    }
}