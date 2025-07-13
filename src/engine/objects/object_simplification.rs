use crate::engine::objects::game_object_core::GameObjectCore;
use crate::engine::objects::game_box::GameBox;
/**
 * Represents a simplified object.
 * This can be passed arount.
 */
pub struct ObjectSimplification {
    pub id: u32,
    pub position: GameBox,
    pub faction: u32,
    pub damage: f32,
    pub name: String,
}


impl ObjectSimplification{
    pub fn new(core: &GameObjectCore) -> ObjectSimplification {
        return ObjectSimplification {
            id: core.id,
            position: GameBox::new(core.get_x(), core.get_y(), core.get_width(), core.get_height()),
            faction: core.faction,
            damage: core.damage,
            name: core.name.to_owned(),
        }
    }
}
