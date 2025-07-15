use crate::engine::objects::drawable::engine_animations::ANIMATION_ATTACK;
use crate::engine::objects::drawable::engine_animations::ANIMATION_IDLE;
use crate::engine::objects::drawable::game_sprite::GameSprite;
use crate::game::enums::drawing_layers::DrawingLayer;
use crate::{engine::{drawing::drawing_manager::{DrawingManager}, events::{event_manager::{EventManager}, game_event::GameEvent}, objects::{drawable::game_object_animation::GameObjectAnimation, spawning::object_summon::ObjectSummon}, utils::timer::Timer}, game::{entities::factions::FACTION_PLAYER}};

pub struct ObjectWeapon {
    // stats
    attack_timer: Timer,
    
    // logic
    sprite: GameSprite,
    weapon_offset: f32,

}

impl ObjectWeapon {
    pub fn new(fire_rate: u128, sprite: &str) -> ObjectWeapon {
        return ObjectWeapon { 
            attack_timer: Timer::new(fire_rate), 
            sprite: GameSprite::new(0.0, 0.0, sprite, DrawingLayer::GameObjects.get_value()).enable_flipping_with_rotation(),
            weapon_offset: 16.0,
        }
    }

    pub fn draw(&self, drawing_manager: &mut DrawingManager) {
        // TODO: flip upside down when facing left
        self.sprite.draw(drawing_manager);
    }

    pub fn update(&mut self, holder_x: f32, holder_y: f32) {
        // position
        self.sprite.position.x = holder_x + (self.sprite.get_rotation().cos() * self.weapon_offset);
        self.sprite.position.y = holder_y + (self.sprite.get_rotation().sin() * self.weapon_offset);


        // animation
        if self.attack_timer.can_activate() {
            self.sprite.play_animation(ANIMATION_IDLE, false);
        }else {
            self.sprite.play_animation(ANIMATION_ATTACK, false);
        }
    }

    pub fn fire(&mut self, event_manager: &mut EventManager) {
        if self.attack_timer.can_activate() {
            self.attack_timer.activate();
            event_manager.push_event(GameEvent::SpawnObject 
                { 
                    summon: ObjectSummon::new("projectile", self.sprite.position.x, self.sprite.position.y)
                    .set_direction(self.sprite.get_rotation())
                    .set_speed(4.0)
                    .set_faction(FACTION_PLAYER)
                    .set_damage(10.0)
                    .set_sprite("bow_0001")
                }
            );
            self.sprite.play_animation(ANIMATION_ATTACK, true);
            
        }
    }

    pub fn set_direction(&mut self, direction: f32) {
        self.sprite.set_rotation(direction);
    }
    pub fn add_animation(mut self, index: i32, animation: GameObjectAnimation) -> ObjectWeapon {
        self.sprite.add_animation(index, animation);
        return self;
    }

    pub fn set_weapon_offset(mut self, offset: f32) -> ObjectWeapon {
        self.weapon_offset = offset;
        return self;
    }
}