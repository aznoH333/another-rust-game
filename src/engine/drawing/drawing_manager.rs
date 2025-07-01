
use std::{collections::HashMap, env, path::Path};

use ggez::{graphics::{Canvas, DrawParam, Image, InstanceArray}, Context};


use crate::utils::file_utils::FileUtils;

use super::{draw_buffer_data::DrawBufferData, drawing_context::DrawingContext};




pub struct DrawingManager{
    sprites: HashMap<String, Image>,
    draw_buffer: Vec<DrawBufferData>,
    drawing_context: DrawingContext,
    // drawing layer to batch map
    draw_batches: HashMap<String, HashMap<i32, InstanceArray>>,
    drawing_layers: Vec<i32>,
}

impl DrawingManager{
    pub fn new(context: &mut Context, drawing_layers: Vec<i32>) -> DrawingManager {
        let mut output = DrawingManager{
            sprites: HashMap::new(),
            draw_buffer: Vec::new(),
            drawing_context: DrawingContext::new(&context),
            draw_batches: HashMap::new(),
            drawing_layers: drawing_layers,
        };
        // load sprites
        output.load_sprites_in_folder(Path::new("./sprites/"), context);

        return output;

    }

    fn load_sprites_in_folder(&mut self, folder: &Path, context: &mut Context){
        // move current context to assets
        // this is done because of ggez resource paths
        env::set_current_dir("./assets/").unwrap();
        let folder_contents = FileUtils::get_files_in_folder(folder, &String::from("png"));
        env::set_current_dir("../").unwrap();

        for file in folder_contents {
            // shitty hack to remove first char
            // ggez directory system is very strict about its file paths
            let mut file_path = file.as_path().to_str().to_owned().unwrap().chars();
            file_path.next().unwrap();


            let file_name = file.file_name().unwrap().to_str().unwrap();
            
            let sprite_name = file_name[..file_name.len()-4].to_owned();
            let image = Image::from_path(context, file_path.as_str()).unwrap();
            self.sprites.insert(
                sprite_name.to_owned(), 
                image.to_owned(),
            );

            self.draw_batches.insert(sprite_name.to_owned(), HashMap::new());

            // init draw batches
            for layer in &self.drawing_layers {
                self.draw_batches.get_mut(&sprite_name).unwrap().insert(layer.clone(), InstanceArray::new(context, image.to_owned()));
            }
        }
    }

    pub fn draw_sprite(&mut self, sprite_name: &String, x: f32, y: f32, z_index: i32, scale: f32, fliped: bool, rotation: f32){
        self.draw_buffer.push(DrawBufferData::new(sprite_name.clone(), x, y, z_index, scale, fliped, rotation));
    }

    pub fn draw_buffer_to_canvas(&mut self, canvas: &mut Canvas){
        // collect to draw batches
        for draw_data in &self.draw_buffer{
            let sprite = self.sprites.get(draw_data.get_sprite_name()).expect(format!("Sprite not found {}", draw_data.get_sprite_name()).as_str());
            let target_batch = self.draw_batches.get_mut(draw_data.get_sprite_name()).expect(format!("Sprite not found {}", draw_data.get_sprite_name()).as_str());//.push(draw_data.convert_to_draw_param(&self.drawing_context));
            
            target_batch.get_mut(&draw_data.get_z_index()).unwrap().push(draw_data.convert_to_draw_param(&self.drawing_context, sprite.width(), sprite.height()));
        }

        // draw batches
        for layer in &self.drawing_layers{

            for draw_batch in &mut self.draw_batches{
                
                let current_draw_batch = draw_batch.1.get_mut(&layer).unwrap();
                
                canvas.draw(current_draw_batch, DrawParam::default());
                current_draw_batch.clear();
            }
        }


        self.draw_buffer.clear();
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