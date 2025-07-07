use crate::{engine::{events::{event_manager::EventManager, game_event::GameEvent}, objects::object_summon::ObjectSummon, world::{world_constants::TILE_SIZE, world_manager::WorldManager}}, utils::{number_utils::NumberUtils, space_utils::SpaceUtils}};

use super::room::Room;

#[derive(PartialEq)]
pub enum PointOfInterest{
    None = 0,
    PlayerSpawn,
    Exit,
    Button,
    Shop,
    Treasure,
    BigFight,
}
// entities have their x,y set to their center. when spawning new entities they have to be shifted +half tile to be grid aligned
const SPAWN_OFFSET: f32 = TILE_SIZE as f32 / 2.0;

impl PointOfInterest{
    pub fn populate_room(&self, room: &Room, world_manager: &mut WorldManager, event_manager: &mut EventManager) {
        match self {
            PointOfInterest::None => { self.populate_with_enemies(room, event_manager);  },
            PointOfInterest::PlayerSpawn => self.populate_player_spawn(room, event_manager),
            PointOfInterest::Exit => { self.populate_exit_room(room, event_manager); self.populate_with_enemies(room, event_manager); },
            PointOfInterest::Button => {},
            PointOfInterest::Shop => {self.spawn_shop(room, world_manager, event_manager);},
            PointOfInterest::Treasure => { self.populate_with_enemies(room, event_manager); self.populate_with_treasure(room, event_manager); },
            PointOfInterest::BigFight => { self.populate_with_enemies(room, event_manager); self.populate_with_enemies(room, event_manager); },
        }
    }

    fn populate_player_spawn(&self, room: &Room, event_manager: &mut EventManager){
        let (x, y) = room.pick_random_empty_spot_with_distance( 0.80);
        
        // spawn entities
        event_manager.push_event( GameEvent::SpawnObject { summon: ObjectSummon::new("player", x, y) });
        //event_manager.push_event( GameEvent::SpawnObject { spawn_function: Box::new(move |game_object_manager| {
        //    game_object_manager.add_object(Player::new(x + SPAWN_OFFSET, y + SPAWN_OFFSET));
        //}) });
    }

    fn populate_exit_room(&self, room: &Room, event_manager: &mut EventManager){
        let (x, y) = room.pick_random_empty_spot_with_distance( 0.80);
        //event_manager.push_event( GameEvent::SpawnObject { spawn_function: Box::new(move |game_object_manager|{
        //    game_object_manager.add_object(ExitStairs::new(x + SPAWN_OFFSET, y + SPAWN_OFFSET));
        //})});
    }

    fn populate_with_enemies(&self, room: &Room, event_manager: &mut EventManager){
        let ammount_of_enemies_to_spawn = room.get_surface() / 40 + 1;

        for _ in 0..ammount_of_enemies_to_spawn {
            let (x, y) = room.pick_random_empty_spot_in_room();

            //event_manager.push_event( GameEvent::SpawnObject { spawn_function: Box::new(move |game_object_manager|{
            //    game_object_manager.add_object(Gremlin::new(x + SPAWN_OFFSET, y + SPAWN_OFFSET));
            //})});
        }
    }

    fn populate_with_treasure(&self, room: &Room, event_manager: &mut EventManager){
        let ammount_of_treasure_to_spawn = NumberUtils::random_integer(1, 3);

        for _ in 0..ammount_of_treasure_to_spawn {
            let (x, y) = room.pick_random_empty_spot_with_distance( 0.05);
            //event_manager.push_event( GameEvent::SpawnObject { spawn_function: Box::new(move |game_object_manager|{
            //    game_object_manager.add_object(Treasure::new(x + SPAWN_OFFSET, y + SPAWN_OFFSET));
            //})});
        }
    }

    fn spawn_shop(&self, room: &Room, world_manager: &mut WorldManager, event_manager: &mut EventManager){
        for _ in 0..100 {
            let (x, y) = room.pick_random_empty_spot_with_distance( 0.05);

            if world_manager.is_space_empty(SpaceUtils::game_units_to_world_units(x) - 1, SpaceUtils::game_units_to_world_units(y), 3 , 3) {
                //event_manager.push_event( GameEvent::SpawnObject { spawn_function: Box::new(move |game_object_manager|{
                //    game_object_manager.add_object(ShopKeeper::new(x + SPAWN_OFFSET, y + SPAWN_OFFSET));
                //    game_object_manager.add_object(ShopItem::new(x - TILE_SIZE as f32 + SPAWN_OFFSET, y + TILE_SIZE as f32 + SPAWN_OFFSET));
                //    game_object_manager.add_object(ShopItem::new(x + TILE_SIZE as f32 + SPAWN_OFFSET, y + TILE_SIZE as f32 + SPAWN_OFFSET));
                //})});
                break;
            }
        }
        
    }
}