use crate::{engine::{objects::{game_object::{GameObject, GameObjectBuilder}, spawning::object_summon::{ObjectSummonParameters, ObjectSummonRegistration}}, types::controller_type::CONTROLLER_TYPE_UPDATE}, game::{entities::{controllers::fade_away::FadeAwayController, objects::effects::giblet_type::{GIBLET_BLOB, GIBLET_SPLINTER}}, enums::drawing_layers::DrawingLayer}, utils::{number_utils::NumberUtils, vec_utils::VecUtils}};
use std::f32::consts::PI;

fn giblet_new(parameters: &ObjectSummonParameters) -> GameObject {
    let mut sprite_name = "";

    if parameters.object_type == GIBLET_BLOB {
        let rust = vec!("giblets_0001", "giblets_0002", "giblets_0003");
        sprite_name = VecUtils::pick_random_element_vec(&rust).to_owned();
    }else if parameters.object_type == GIBLET_SPLINTER {
        let rust = vec!("giblets_0004", "giblets_0005");
        sprite_name = VecUtils::pick_random_element_vec(&rust).to_owned();
    }

    let direction = NumberUtils::random_float_range(0.0, PI*2.0);
    let x_m = direction.cos() * parameters.speed;
    let y_m = direction.sin() * parameters.speed;
    
    return 
    GameObjectBuilder::new(parameters.x, parameters.y, sprite_name, DrawingLayer::Effects.get_value())
    .disable_auto_flipping()
    .set_dimensions(8.0, 8.0)
    .set_sprite_offset(-8.0, -8.0)
    .set_bounciness(0.95)
    .set_starting_velocity(x_m, y_m)
    .set_color(parameters.color)
    .add_controller(CONTROLLER_TYPE_UPDATE, Box::new(FadeAwayController::new(2000)))
    .build()
}

inventory::submit! {
    ObjectSummonRegistration::new("giblet", giblet_new)
}