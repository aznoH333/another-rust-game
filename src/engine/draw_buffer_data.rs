use ggez::{glam::Vec2, graphics::{self, Canvas, Image}};

pub struct DrawBufferData{
    x: f32,
    y: f32,
    sprite_name: String,
    z_index: i32,
}


impl DrawBufferData{
    pub fn new(sprite_name: String, x: f32, y: f32, z_index: i32) -> DrawBufferData{
        return DrawBufferData{
            sprite_name: sprite_name,
            x: x,
            y: y,
            z_index: z_index,
        }
    }

    fn convert_to_draw_param(&self) -> graphics::DrawParam {
        return graphics::DrawParam::new().dest(Vec2::new(self.x, self.y)).z(self.z_index);
    }

    pub fn draw(&self, image: &Image, canvas: &mut Canvas){
        canvas.draw(image, self.convert_to_draw_param());
    }

    pub fn get_sprite_name(&self) -> &str {
        return &self.sprite_name
    }
}