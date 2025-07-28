use ggez::graphics::Canvas;
use ggez::Context;

use crate::utils::number_utils::NumberUtils;

use super::{camera::Camera, screen_context::ScreenContext};

pub struct DrawingContext{
    camera: Camera,
    screen_context: ScreenContext,
    
}


impl DrawingContext {
    pub fn new(
        context: &Context, 
        game_screen_w: f32,
        game_screen_h: f32,
    ) -> DrawingContext {
        let mut output = DrawingContext{
            camera: Camera::new(0.0, 0.0, 1.0), 
            screen_context: ScreenContext::new(context, game_screen_w, game_screen_h),
            
        };


        return output;
    }

    // camer operations
    pub fn get_sprite_x_offset(&self, is_static: bool) -> f32 {
        if is_static {
            return self.screen_context.get_screen_left()
        }else {
            return -self.camera.get_left_offset(&self.screen_context);
        }
    }

    pub fn get_sprite_y_offset(&self, is_static: bool) -> f32 {
        if is_static {
            return self.screen_context.get_screen_top()
        }else {
            return -self.camera.get_top_offset(&self.screen_context)
        }
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
        // TODO : fix this
        // self.screen_context = ScreenContext::new(context);
    }

    pub fn set_up_canvas(&mut self, canvas: &mut Canvas) {
        self.screen_context.set_up_canvas(canvas);
    }


}