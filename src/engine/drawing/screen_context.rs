use ggez::graphics::Canvas;
use ggez::graphics::Rect;
use ggez::graphics::Sampler;
use ggez::Context;

pub struct ScreenContext{
    width: f32,
    height: f32,
    target_w: f32,
    target_h: f32,
    
    drawing_area_w: f32,
    drawing_area_h: f32,
    screen_space: Rect,

    screen_offset_left: f32,
    screen_offset_top: f32,
}


impl ScreenContext{
    pub fn new(context: &Context, 
        game_screen_w: f32,
        game_screen_h: f32) -> ScreenContext{
        // calculate draw area
            
        let size = context.gfx.drawable_size();

        let screen_width = size.0;
        let screen_height = size.1;

        
        // Calculate scale
        let scale_x = game_screen_w / screen_width;
        let scale_y = game_screen_h / screen_height;

        let scale = scale_x.max(scale_y);

        let new_width = screen_width * scale;
        let new_height = screen_height * scale;

        // calculate screen area
        let screen_area_width = game_screen_w / scale;
        let screen_area_height = game_screen_h / scale;

        let screen_area_offset_x = (new_width - game_screen_w) / scale / 2.0; // (screen_width - 256.0) / 2.0 / scale;
        let screen_area_offset_y = (new_height - game_screen_h) / scale / 2.0; // (screen_width - 256.0) / 2.0 / scale;


        return ScreenContext{
            width: screen_width,
            height: screen_height,
            target_w: game_screen_w, // drawing_area_w as f32,
            target_h: game_screen_h,// drawing_area_h as f32,
            drawing_area_w: new_width,
            drawing_area_h: new_height,
            screen_space: Rect::new(screen_area_offset_x, screen_area_offset_y, screen_area_width, screen_area_height),
            screen_offset_left: (new_width - game_screen_w) / 2.0,
            screen_offset_top: (new_height - game_screen_h) / 2.0
        }


    }

    pub fn get_width(&self) -> f32{
        return self.drawing_area_w;
    }

    pub fn get_height(&self) -> f32{
        return self.drawing_area_h;
    }

    pub fn get_screen_left(&self) -> f32 {
        return self.screen_offset_left;
    }

    pub fn get_screen_top(&self) -> f32 {
        return self.screen_offset_top;
    }

    pub fn set_up_canvas(&mut self, canvas: &mut Canvas){
        canvas.set_sampler(Sampler::nearest_clamp());
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, self.drawing_area_w, self.drawing_area_h));
        canvas.set_scissor_rect(self.screen_space);
    }
}