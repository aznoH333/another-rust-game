use crate::utils::space_utils::SpaceUtils;


pub struct GameBox{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl GameBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> GameBox{
        return GameBox {
            x: x,
            y: y,
            width: width,
            height: height,
        };
    }
    
    /**
     * returns the "left" side of object -> x position
     */
    pub fn left(&self) -> f32 {
        return self.x - (self.width / 2.0);
    }

    /**
     * returns the "right" side of object -> x position
     */
    pub fn right(&self) -> f32 {
        return self.x + (self.width / 2.0);
    }

    /**
     * returns the "top" side of object -> y position
     */
    pub fn top(&self) -> f32 {
        return self.y - (self.height / 2.0);
    }

    /**
     * returns the "bottom" side of object -> y position
     */
    pub fn bottom(&self) -> f32 {
        return self.y + (self.height / 2.0);
    }

    pub fn collides_with_box(&self, other: &GameBox) -> bool {
        return SpaceUtils::squares_collide_f32(
            self.left(), 
            self.top(), 
            self.width, 
            self.height, 
            other.left(), 
            other.top(), 
            other.width, 
            other.height);
    }
}