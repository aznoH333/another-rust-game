use crate::{engine::objects::{game_object::GameObject, game_object_controller::GameObjectController, game_object_core::GameObjectCore}, game::enums::drawing_layers::DrawingLayer};

pub struct Player{

}



impl Player{
    pub fn new(x: f32, y: f32) -> GameObject{


        let controller = Player{

        };
        let core = GameObjectCore::new(x, y, "player_0001.png", DrawingLayer::PLAYER as i32);
        

        return GameObject::new(
            core,
            Box::new(controller)
        );

    }
}


impl GameObjectController for Player{
    fn update(&mut self, core: &mut GameObjectCore) {
        core.x += 0.1;
    }

}