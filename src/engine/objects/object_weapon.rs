use ggez::graphics::Color;

use crate::{engine::{drawing::drawing_manager::{self, DrawingManager}, events::{event_manager::{self, EventManager}, game_event::GameEvent}, objects::{drawable::game_object_animation::GameObjectAnimation, spawning::object_summon::ObjectSummon}, utils::timer::Timer}, game::{entities::factions::FACTION_PLAYER, enums::drawing_layers::DrawingLayer}};
use std::collections::HashMap;

pub struct ObjectWeapon {
    // stats
    attack_timer: Timer,
    
    // logic
    direction: f32,
    x: f32,
    y: f32,

    // drawing
    sprite: String,
    animations: HashMap<i32, GameObjectAnimation>,
    use_animations: bool,


}

impl ObjectWeapon {
    pub fn new(fire_rate: u128, sprite: &str) -> ObjectWeapon {
        return ObjectWeapon { 
            attack_timer: Timer::new(fire_rate), 
            animations: HashMap::new(),
            direction: 0.0,
            x: 0.0,
            y: 0.0,
            sprite: sprite.to_owned(),
            use_animations: false,
        }
    }

    pub fn draw(&self, drawing_manager: &mut DrawingManager) {
        // TODO: animation
        // TODO: flip upside down when facing left
        drawing_manager.draw_sprite(&self.sprite.to_owned(), self.x, self.y, DrawingLayer::WorldObjects.get_value(), 1.0, false, self.direction, Color::WHITE);
    }

    pub fn update(&mut self, holder_x: f32, holder_y: f32) {
        self.x = holder_x;
        self.y = holder_y;

    }

    pub fn fire(&mut self, event_manager: &mut EventManager) {
        if self.attack_timer.can_activate() {
            self.attack_timer.activate();
            event_manager.push_event(GameEvent::SpawnObject 
                { 
                    summon: ObjectSummon::new("projectile", self.x, self.y)
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
    // man would be cool if there was some way to do inheritence
    // could split game object core into smaller parts but thats maybe too much work? TODO : think about this
    // builder methods
    pub fn add_animation(mut self, index: i32, animation: GameObjectAnimation) -> ObjectWeapon {
        self.use_animations = true;
        self.animations.insert(index, animation);
        return self;
    }
}