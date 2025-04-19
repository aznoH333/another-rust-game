use std::collections::HashMap;

use rand::seq::{IndexedRandom, IteratorRandom};

pub fn pick_random_element_vec<T>(vec: &Vec<T>) -> &T {
    let mut rng = rand::rng();

    return vec.choose(&mut rng).unwrap();
}

pub fn pick_random_key_map<T, V>(map: &HashMap<T, V>) -> &T {
    let mut rng = rand::rng();

    return map.keys().choose(&mut rng).unwrap();
}