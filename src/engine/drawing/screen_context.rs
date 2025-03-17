use ggez::Context;

pub struct ScreenContext{
    width: f32,
    height: f32,
}


impl ScreenContext{
    pub fn new(context: &Context) -> ScreenContext{
        let size = context.gfx.drawable_size();
        return ScreenContext{
            width: size.0,
            height: size.1
        }
    }

    pub fn get_width(&self) -> f32{
        return self.width;
    }

    pub fn get_height(&self) -> f32{
        return self.height;
    }
}