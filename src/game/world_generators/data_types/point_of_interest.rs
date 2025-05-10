use crate::{engine::{events::{event_manager::EventManager, game_event::GameEvent}, objects::game_object_manager::GameObjectManager, world::world_manager::WorldManager}, game::entities::objects::{exit_stairs::ExitStairs, player::Player}};

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

impl PointOfInterest{
    pub fn populate_room(&self, room: &Room, world_manager: &mut WorldManager, event_manager: &mut EventManager) {
        match self {
            PointOfInterest::None => {},
            PointOfInterest::PlayerSpawn => self.populate_player_spawn(room, event_manager),
            PointOfInterest::Exit => self.populate_exit_room(room, event_manager),
            PointOfInterest::Button => {},
            PointOfInterest::Shop => {},
            PointOfInterest::Treasure => {},
            PointOfInterest::BigFight => {},
        }
    }

    fn populate_player_spawn(&self, room: &Room, event_manager: &mut EventManager){
        let (x, y) = room.pick_random_empty_spot_with_distance( 0.80);
        
        // spawn entities
        event_manager.push_event( GameEvent::SpawnObject { spawn_function: Box::new(move |game_object_manager| {
            game_object_manager.add_object(Player::new(x, y));
        }) });
    }

    fn populate_exit_room(&self, room: &Room, event_manager: &mut EventManager){
        let (x, y) = room.pick_random_empty_spot_with_distance( 0.80);
        event_manager.push_event( GameEvent::SpawnObject { spawn_function: Box::new(move |game_object_manager|{
            game_object_manager.add_object(ExitStairs::new(x, y));
        })});
    }
}