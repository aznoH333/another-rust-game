use ggez::Context;

use super::{camera::Camera, screen_context::ScreenContext};

pub struct DrawingContext{
    camera: Camera,
    screen_context: ScreenContext,
}


impl DrawingContext {
    pub fn new(context: &Context) -> DrawingContext {
        return DrawingContext{
            camera: Camera::new(0.0, 0.0, 1.0), 
            screen_context: ScreenContext::new(context)
        }
    }

    // camer operations
    pub fn get_sprite_x_offset(&self) -> f32 {
        return self.camera.get_left_offset(&self.screen_context);
    }

    pub fn get_sprite_y_offset(&self) -> f32 {
        return self.camera.get_top_offset(&self.screen_context);
    }

    pub fn get_scale(&self) -> f32 {
        return self.camera.get_scale();
    }

    pub fn get_camera_x(&self) -> f32 {
        return self.camera.get_x();
    }

    pub fn get_camera_y(&self) -> f32 {
        return self.camera.get_y();
    }

    pub fn set_camera_target(&mut self, x: f32, y: f32) {
        self.camera.set_target(x, y);
    }

    pub fn set_camera_zoom(&mut self, zoom: f32){
        self.camera.set_zoom(zoom);
    }


    // context operations
    pub fn reload_context(&mut self, context: &Context){
        self.screen_context = ScreenContext::new(context);
    }

}