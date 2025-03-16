
use std::{collections::HashMap, env, path::Path};

use ggez::{graphics::{Canvas, Image}, Context};

use crate::utils::file_utils::get_files_in_folder;

use super::draw_buffer_data::DrawBufferData;




pub struct SpriteManager{
    sprites: HashMap<String, Image>,
    draw_buffer: Vec<DrawBufferData>
}

impl SpriteManager{
    pub fn new(context: &mut Context) -> SpriteManager {
        let mut output = SpriteManager{
            sprites: HashMap::new(),
            draw_buffer: Vec::new(),
        };
        // load sprites
        output.load_sprites_in_folder(Path::new("./sprites/"), context);

        return output;

    }

    fn load_sprites_in_folder(&mut self, folder: &Path, context: &mut Context){
        env::set_current_dir("./assets/").unwrap();
        let folder_contents = get_files_in_folder(folder, &String::from("png"));
        env::set_current_dir("../").unwrap();

        for file in folder_contents {
            // shitty hack to remove first char
            let mut filePath = file.as_path().to_str().to_owned().unwrap().chars();
            filePath.next().unwrap();

            
            self.sprites.insert(
                file.file_name().unwrap().to_str().unwrap().to_owned(), 
                Image::from_path(context, filePath.as_str()).unwrap());
        }
    }

    pub fn draw_sprite(&mut self, sprite_name: String, x: f32, y: f32, z_index: i32){
        self.draw_buffer.push(DrawBufferData::new(sprite_name, x, y, z_index));
    }

    pub fn draw_buffer_to_canvas(&mut self, canvas: &mut Canvas){

        for draw_data in &self.draw_buffer{
            let image = self.sprites.get(draw_data.get_sprite_name()).unwrap();

            draw_data.draw(image, canvas);                
        }     
    }
}