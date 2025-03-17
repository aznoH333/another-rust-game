
use std::{collections::HashMap, env, path::Path};

use ggez::{graphics::{Canvas, Image}, Context};

use crate::utils::file_utils::get_files_in_folder;

use super::{draw_buffer_data::DrawBufferData, drawing_context::DrawingContext};




pub struct DrawingManager{
    sprites: HashMap<String, Image>,
    draw_buffer: Vec<DrawBufferData>,
    drawing_context: DrawingContext,
}

impl DrawingManager{
    pub fn new(context: &mut Context) -> DrawingManager {
        let mut output = DrawingManager{
            sprites: HashMap::new(),
            draw_buffer: Vec::new(),
            drawing_context: DrawingContext::new(&context),
        };
        // load sprites
        output.load_sprites_in_folder(Path::new("./sprites/"), context);

        return output;

    }

    fn load_sprites_in_folder(&mut self, folder: &Path, context: &mut Context){
        // move current context to assets
        // this is done because of ggez resource paths
        env::set_current_dir("./assets/").unwrap();
        let folder_contents = get_files_in_folder(folder, &String::from("png"));
        env::set_current_dir("../").unwrap();

        for file in folder_contents {
            // shitty hack to remove first char
            // ggez directory system is very strict about its file paths
            let mut file_path = file.as_path().to_str().to_owned().unwrap().chars();
            file_path.next().unwrap();

            
            self.sprites.insert(
                file.file_name().unwrap().to_str().unwrap().to_owned(), 
                Image::from_path(context, file_path.as_str()).unwrap());
        }
    }

    pub fn draw_sprite(&mut self, sprite_name: String, x: f32, y: f32, z_index: i32, scale: f32){
        self.draw_buffer.push(DrawBufferData::new(sprite_name, x, y, z_index, scale));
    }

    pub fn draw_buffer_to_canvas(&mut self, canvas: &mut Canvas){
        for draw_data in &self.draw_buffer{
            let image = self.sprites.get(draw_data.get_sprite_name()).unwrap();

            draw_data.draw(image, canvas, &self.drawing_context);                
        }     
    }

    pub fn reload_context(&mut self, context: &Context){
        self.drawing_context.reload_context(context);
    }

    // context functions
    pub fn get_camera_x(&self) -> f32 {
        return self.drawing_context.get_camera_x();
    }

    pub fn get_camera_y(&self) -> f32 {
        return self.drawing_context.get_camera_y();
    }

    pub fn set_camera_target(&mut self, x: f32, y: f32){
        self.drawing_context.set_camera_target(x, y);
    }

    pub fn set_camera_zoom(&mut self, zoom: f32){
        self.drawing_context.set_camera_zoom(zoom);
    }
}