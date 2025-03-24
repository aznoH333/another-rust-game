mod utils;
mod engine;
mod game;


use std::{env, path};
use engine::drawing::drawing_manager::DrawingManager;
use engine::input::input::InputHandler;
use engine::objects::game_object_manager::GameObjectManager;
use engine::world::world_manager::WorldManager;
use game::entities::player::Player;
use ggez::input::keyboard::KeyCode;
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
    world_manager: WorldManager,
    input: InputHandler,
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
            world_manager: WorldManager::new(),
            input: InputHandler::new(),
        }
    }
}

impl EventHandler for MyGame {
   fn update(&mut self, _context: &mut Context) -> GameResult {
        // Update code here...
        self.game_object_manager.update(&mut self.sprite_manager, &self.input, &self.world_manager);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(context, Color::BLACK);
        canvas.set_sampler(Sampler::nearest_clamp());

        self.world_manager.draw_world(&mut self.sprite_manager);


        self.sprite_manager.draw_buffer_to_canvas(&mut canvas);

        return canvas.finish(context);
    }

    fn key_down_event(
            &mut self,
            ctx: &mut Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> Result<(), ggez::GameError> {
        self.input.handle_keyboard_down_events(input);
        if input.keycode == Some(KeyCode::Escape) {
            ctx.quit_requested = true;
        }
        return Ok(());
    }


    fn key_up_event(&mut self, _ctx: &mut Context, input: ggez::input::keyboard::KeyInput) -> Result<(), ggez::GameError> {
        self.input.handle_keyboard_up_events(input);
        return Ok(());
    }

}