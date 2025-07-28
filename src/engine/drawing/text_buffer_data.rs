use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::graphics::Text;

use crate::engine::drawing::drawing_context::DrawingContext;
use crate::utils::number_utils::NumberUtils;

pub struct GameText {
    text: Text,
    x: f32,
    y: f32,
    use_camera: bool,
    z_index: i32,
    color: Color,
    scale: f32
}


impl GameText {
    pub fn new(text: &str, x: f32, y: f32, z_index: i32) -> GameText {
        return GameText { 
            text: Text::new(text), 
            x: x, 
            y: y, 
            use_camera: true,
            z_index: z_index,
            color: Color::WHITE,
            scale: 1.0,
        }
    }

    pub fn convert_to_draw_param(&self, drawing_context: &DrawingContext) -> DrawParam {
    return DrawParam::new()
        .dest(Vec2::new(
            self.x * drawing_context.get_scale() - (drawing_context.get_sprite_x_offset(true) * NumberUtils::bool_to_f32(self.use_camera)),
            self.y * drawing_context.get_scale() - (drawing_context.get_sprite_y_offset(true) * NumberUtils::bool_to_f32(self.use_camera))))
        .z(self.z_index)
        .color(self.color)
        .scale(Vec2::new(self.scale * drawing_context.get_scale(), self.scale * drawing_context.get_scale()));
    }

    pub fn draw(&self, canvas: &mut Canvas, drawing_context: &DrawingContext) {
        canvas.draw(&self.text, self.convert_to_draw_param(drawing_context));
    }

    pub fn set_color(mut self, color: Color) -> GameText {
        self.color = color;
        return self;
    }

    pub fn make_static(mut self) -> GameText {
        self.use_camera = false;
        return self;
    }

    pub fn set_scale(mut self, scale: f32) -> GameText {
        self.scale = scale;
        return self;
    }
}

/*
TODO : load font
ctx.gfx.add_font(
            "Fancy font",
            graphics::FontData::from_path(ctx, "/Tangerine_Regular.ttf")?,
        );
*/