use super::screen_context::ScreenContext;


pub struct Camera{
    target_x: f32,
    target_y: f32,
    zoom: f32,
}

impl Camera{
    pub fn new(x: f32, y: f32, zoom: f32) -> Camera{
        return Camera{
            target_x: x,
            target_y: y,
            zoom: zoom,
        }
    }

    pub fn get_left_offset(&self, screen_context: &ScreenContext) -> f32 {
        return -screen_context.get_width() / 2.0 + (self.target_x * self.zoom);
    }


    pub fn get_top_offset(&self, screen_context: &ScreenContext) -> f32 {
        return -screen_context.get_height() / 2.0 + (self.target_y * self.zoom);
    }

    pub fn get_scale(&self) -> f32 {
        return self.zoom;
    }

    pub fn get_x(&self) -> f32 {
        return self.target_x;
    }

    pub fn get_y(&self) -> f32 {
        return self.target_y;
    }

    pub fn set_target(&mut self, x: f32, y: f32) {
        self.target_x = x;
        self.target_y = y;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }
}