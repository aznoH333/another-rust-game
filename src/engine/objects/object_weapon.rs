use crate::engine::{objects::game_object_animation::GameObjectAnimation, utils::timer::Timer};
use std::collections::HashMap;

pub struct ObjectWeapon {
    pub attack_timer: Timer,
    pub animations: HashMap<i32, GameObjectAnimation>,
    // TODO this
}