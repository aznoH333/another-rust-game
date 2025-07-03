use crate::engine::objects::game_object_core::GameObjectCore;
use crate::engine::objects::game_box::GameBox;
/**
 * Represents a simplified object.
 * This can be passed arount.
 */
pub struct ObjectSimplification {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub faction: u32,
    pub damage: f32,
    pub width: f32,
    pub height: f32,
    pub name: String,
}


impl ObjectSimplification{
    pub fn new(core: &GameObjectCore) -> ObjectSimplification {
        return ObjectSimplification {
            id: core.id,
            x: core.x,
            y: core.y,
            faction: core.faction,
            damage: core.damage,
            width: core.width,
            height: core.height,
            name: core.name.to_owned(),
        }
    }
}

impl GameBox for ObjectSimplification {
    /**
     * returns the "left" side of object -> x position
     */
    fn left(&self) -> f32 {
        return self.x - (self.width / 2.0);
    }

    /**
     * returns the "right" side of object -> x position
     */
    fn right(&self) -> f32 {
        return self.x + (self.width / 2.0);
    }

    /**
     * returns the "top" side of object -> y position
     */
    fn top(&self) -> f32 {
        return self.y - (self.height / 2.0);
    }

    /**
     * returns the "bottom" side of object -> y position
     */
    fn bottom(&self) -> f32 {
        return self.y + (self.height / 2.0);
    }

    fn get_x(&self) -> f32 {
        return self.x;
    } 

    fn get_y(&self) -> f32 {
        return self.y;
    }

    fn get_width(&self) -> f32 {
        return self.width;
    }

    fn get_height(&self) -> f32 {
        return self.height;
    }

    fn get_id(&self) -> u32 {
        return self.id;
    }
}