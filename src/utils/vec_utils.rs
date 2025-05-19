use std::collections::HashMap;

use rand::seq::{IndexedRandom, IteratorRandom};

use super::number_utils::NumberUtils;


pub struct VecUtils{

}

impl VecUtils{
    pub fn pick_random_element_vec<T>(vec: &Vec<T>) -> &T {
        let mut rng = rand::rng();

        return vec.choose(&mut rng).unwrap();
    }


    pub fn pick_random_index_vec<T>(vec: &Vec<T>) -> usize {
        return NumberUtils::random_integer(0, vec.iter().count() as i32 - 1) as usize;
    }


    pub fn pick_random_key_map<T, V>(map: &HashMap<T, V>) -> &T {
        let mut rng = rand::rng();

        return map.keys().choose(&mut rng).unwrap();
    }
}

