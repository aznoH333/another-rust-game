
use std::{collections::HashMap, path::Path};

use ggez::{graphics::Image, Context};

use crate::utils::file_utils::get_files_in_folder;




struct SpriteManager{
    sprites: HashMap<String, Image>
}

impl SpriteManager{
    pub fn new(context: &mut Context) -> SpriteManager {
        let mut output = SpriteManager{
            sprites: HashMap::new()
        };


        output.load_sprites_in_folder(Path::new("./"), context);

        return output;

    }

    fn load_sprites_in_folder(&mut self, folder: &Path, context: &mut Context){
        let folder_contents = get_files_in_folder(folder);
    }
}