use ggez::{glam::Vec2, graphics::{self, Canvas, Image}};
use ggez::graphics::Color;

use crate::utils::number_utils::NumberUtils;

use super::drawing_context::DrawingContext;

pub struct DrawBufferData{
    x: f32,
    y: f32,
    sprite_name: String,
    z_index: i32,
    scale: f32,
    fliped: bool,
    rotation: f32,
    color: Color,
    is_static: bool,
}


impl DrawBufferData{
    pub fn new(sprite_name: String, x: f32, y: f32, z_index: i32, scale: f32, fliped: bool, rotation: f32, color: Color, is_static: bool) -> DrawBufferData{
        return DrawBufferData{
            sprite_name: sprite_name,
            x: x,
            y: y,
            z_index: z_index,
            scale: scale,
            fliped: fliped,
            rotation,
            color,
            is_static: is_static,
        }
    }

    pub fn convert_to_draw_param(&self, drawing_context: &DrawingContext, width: u32, height: u32) -> graphics::DrawParam {
        return graphics::DrawParam::new()
            .offset(Vec2::new(width as f32 / 2.0, height as f32 / 2.0))
            .rotation(self.rotation)
            .dest(Vec2::new(
                self.x * drawing_context.get_scale() + drawing_context.get_sprite_x_offset(self.is_static),
                self.y * drawing_context.get_scale() + drawing_context.get_sprite_y_offset(self.is_static)))
            .z(self.z_index)
            .color(self.color)
            .scale(Vec2::new(self.scale * drawing_context.get_scale() * NumberUtils::bool_to_minus_plus_f32(!self.fliped), self.scale * drawing_context.get_scale()));
    }

    pub fn draw(&self, image: &Image, canvas: &mut Canvas, drawing_context: &DrawingContext){
        canvas.draw(image, self.convert_to_draw_param(drawing_context, image.width(), image.height()));
    }

    pub fn get_z_index(&self) -> i32 {
        return self.z_index;
    }

    pub fn get_sprite_name(&self) -> &str {
        return &self.sprite_name
    }
}