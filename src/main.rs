mod utils;
mod engine;


use std::{env, path};

use engine::sprite_manager::SpriteManager;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Image};
use ggez::event::{self, EventHandler};
use ggez::*;

use ggez::glam::*;
use ggez::input::keyboard::KeyInput;




fn main() {
    
    
    // set resource path
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    };
    
    // Make a Context.
    let (mut context, event_loop) = ContextBuilder::new("my_game", "me")
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut context);

    // Run!
    event::run(context, event_loop, my_game);
}

struct MyGame {
    sprite_manager: SpriteManager,
    test: Image
    // Your state here...
}

impl MyGame {
    pub fn new(context: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let test = Image::from_path(context, "/sprites/player_0003.png").unwrap();
        
        return MyGame {
            sprite_manager: SpriteManager::new(context),
            test: test,
        }
    }
}

impl EventHandler for MyGame {
   fn update(&mut self, context: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(context, Color::WHITE);
        

        self.sprite_manager.draw_sprite("player_0001.png".to_owned(), 50.0, 50.0, 1);
        self.sprite_manager.draw_sprite("player_0001.png".to_owned(), 60.0, 50.0, 1);

        self.sprite_manager.draw_buffer_to_canvas(&mut canvas);

        canvas.finish(context)
    }
}