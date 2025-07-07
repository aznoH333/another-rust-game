use crate::{engine::objects::{game_object::{GameObject, GameObjectBuilder}, object_summon::ObjectSummonParameters}, game::{entities::objects::effects::giblet_type::{GIBLET_BLOB, GIBLET_SPLINTER}, enums::drawing_layers::DrawingLayer}, utils::{number_utils::NumberUtils, textures::TextureUtils, vec_utils::VecUtils}};
use std::f32::consts::PI;

fn giblet_new(parameters: &ObjectSummonParameters) -> GameObject {
    let mut sprite_name = "";

    if parameters.object_type == GIBLET_BLOB {
        let rust = vec!("giblet_0001", "giblet_0002", "giblet_0003", "giblet_0004");
        sprite_name = VecUtils::pick_random_element_vec(&rust).to_owned();
    }else if parameters.object_type == GIBLET_SPLINTER {
        let rust = vec!("giblet_0005", "giblet_0006", "giblet_0007", "giblet_0008", "giblet_0009");
        sprite_name = VecUtils::pick_random_element_vec(&rust).to_owned();
    }

    let direction = NumberUtils::random_float_range(0.0, PI);
    let x_m = direction.cos() * parameters.speed;
    let y_m = direction.sin() * parameters.speed;
    
    return 
    GameObjectBuilder::new(parameters.x, parameters.y, sprite_name, DrawingLayer::GameObjects.get_value())
    .disable_auto_flipping()
    .set_dimensions(4.0, 4.0)
    .set_sprite_offset(28.0, 28.0)
    .set_bounciness(0.95)
    .set_friction(0.01)
    .set_starting_velocity(x_m, y_m)
    .build()
}