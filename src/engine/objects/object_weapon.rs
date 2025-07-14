use crate::engine::objects::drawable::game_sprite::GameSprite;
use crate::game::enums::drawing_layers::DrawingLayer;
use crate::{engine::{drawing::drawing_manager::{DrawingManager}, events::{event_manager::{EventManager}, game_event::GameEvent}, objects::{drawable::game_object_animation::GameObjectAnimation, spawning::object_summon::ObjectSummon}, utils::timer::Timer}, game::{entities::factions::FACTION_PLAYER}};

pub struct ObjectWeapon {
    // stats
    attack_timer: Timer,
    
    // logic
    direction: f32,
    sprite: GameSprite,


}

impl ObjectWeapon {
    pub fn new(fire_rate: u128, sprite: &str) -> ObjectWeapon {
        return ObjectWeapon { 
            attack_timer: Timer::new(fire_rate), 
            direction: 0.0,
            sprite: GameSprite::new(0.0, 0.0, sprite, DrawingLayer::World.get_value())
        }
    }

    pub fn draw(&self, drawing_manager: &mut DrawingManager) {
        // TODO: flip upside down when facing left
        self.sprite.draw(drawing_manager);
    }

    pub fn update(&mut self, holder_x: f32, holder_y: f32) {
        self.sprite.position.x = holder_x;
        self.sprite.position.y = holder_y;

    }

    pub fn fire(&mut self, event_manager: &mut EventManager) {
        if self.attack_timer.can_activate() {
            self.attack_timer.activate();
            event_manager.push_event(GameEvent::SpawnObject 
                { 
                    summon: ObjectSummon::new("projectile", self.sprite.position.x, self.sprite.position.y)
                    .set_direction(self.direction)
                    .set_speed(4.0)
                    .set_faction(FACTION_PLAYER)
                    .set_damage(10.0)
                    .set_sprite("bow_0001")
                }
            );
        }
    }

    pub fn set_direction(&mut self, direction: f32) {
        self.direction = direction;
    }
    pub fn add_animation(mut self, index: i32, animation: GameObjectAnimation) -> ObjectWeapon {
        self.sprite.add_animation(index, animation);
        return self;
    }
}