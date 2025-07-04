use crate::utils::space_utils::SpaceUtils;

pub trait GameBox {
    // TODO : rewerite game object core to use composition instead of interface implementation
    fn get_id(&self) -> u32;
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn left(&self) -> f32;
    fn right(&self) -> f32;
    fn top(&self) -> f32;
    fn bottom(&self) -> f32;
    
    
    fn collides_with_box(&self, other: &dyn GameBox) -> bool {
        return SpaceUtils::squares_collide_f32(
            self.left(), 
            self.top(), 
            self.get_width(), 
            self.get_height(), 
            other.left(), 
            other.top(), 
            other.get_width(), 
            other.get_height());
    }

    fn equals(&self, other: &dyn GameBox) -> bool {
        return self.get_id() == other.get_id();
    }
}
