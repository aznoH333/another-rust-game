mod utils;
mod engine;
mod game;


use std::{env, path};
use engine::drawing::drawing_manager::DrawingManager;
use engine::objects::game_object_manager::GameObjectManager;
use game::entities::player::Player;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Sampler};
use ggez::event::{self, EventHandler};


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
    sprite_manager: DrawingManager,
    game_object_manager: GameObjectManager,
    // Your state here...
}

impl MyGame {
    pub fn new(context: &mut Context) -> MyGame {
        // sprite manager
        let mut sprite_manager = DrawingManager::new(context);
        sprite_manager.set_camera_zoom(5.0);

        let mut game_object_manager = GameObjectManager::new();

        game_object_manager.add_object(Player::new(0.0, 0.0));

        return MyGame {
            sprite_manager: sprite_manager,
            game_object_manager: game_object_manager,
            
        }
    }
}

impl EventHandler for MyGame {
   fn update(&mut self, _context: &mut Context) -> GameResult {
        // Update code here...
        self.game_object_manager.update(&mut self.sprite_manager);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(context, Color::BLACK);
        canvas.set_sampler(Sampler::nearest_clamp());




        self.sprite_manager.draw_buffer_to_canvas(&mut canvas);

        canvas.finish(context)
    }
}