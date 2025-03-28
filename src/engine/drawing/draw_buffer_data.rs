use ggez::{glam::Vec2, graphics::{self, BlendMode, Canvas, Image}};

use super::drawing_context::DrawingContext;

pub struct DrawBufferData{
    x: f32,
    y: f32,
    sprite_name: String,
    z_index: i32,
    scale: f32,
}


impl DrawBufferData{
    pub fn new(sprite_name: String, x: f32, y: f32, z_index: i32, scale: f32) -> DrawBufferData{
        return DrawBufferData{
            sprite_name: sprite_name,
            x: x,
            y: y,
            z_index: z_index,
            scale: scale,
        }
    }

    pub fn convert_to_draw_param(&self, drawing_context: &DrawingContext) -> graphics::DrawParam {
        return graphics::DrawParam::new()
            .dest(Vec2::new(self.x * drawing_context.get_scale() - drawing_context.get_sprite_x_offset(), self.y * drawing_context.get_scale() - drawing_context.get_sprite_y_offset()))
            .z(self.z_index)
            .scale(Vec2::new(self.scale * drawing_context.get_scale(), self.scale * drawing_context.get_scale()));
    }

    pub fn draw(&self, image: &Image, canvas: &mut Canvas, drawing_context: &DrawingContext){
        canvas.draw(image, self.convert_to_draw_param(drawing_context));
    }

    pub fn get_z_index(&self) -> i32 {
        return self.z_index;
    }

    pub fn get_sprite_name(&self) -> &str {
        return &self.sprite_name
    }
}