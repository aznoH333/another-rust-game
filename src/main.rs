mod utils;
mod engine;
mod game;


use std::{env, path};
use engine::drawing::drawing_manager::DrawingManager;
use engine::events::event_manager::EventManager;
use engine::input::input::InputHandler;
use engine::objects::game_object_manager::GameObjectManager;
use engine::world::world_manager::WorldManager;
use game::enums::drawing_layers::DrawingLayer;
use game::world_generators::basic_world_generator::BasicRoomGenerator;
use game::world_generators::temes::theme_initializers::blue_dungeon_theme::initialize_blue_dungeon_theme;
use ggez::conf::WindowMode;
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
        .window_mode( WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::Desktop).dimensions(1920.0, 1080.0) )
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
    event_manager: EventManager,
}

impl MyGame {
    pub fn new(context: &mut Context) -> MyGame {
        
        // sprite manager
        let mut sprite_manager = DrawingManager::new(context, Vec::from_iter(DrawingLayer::VALUES.iter().map(|it|{return it.get_value()})));
        sprite_manager.set_camera_zoom(5.0);
        // game object manager
        let game_object_manager = GameObjectManager::new();

        // event manager
        let mut event_manager = EventManager::new();


        // world theme
        let theme = initialize_blue_dungeon_theme();

        // construct output
        return MyGame {
            sprite_manager: sprite_manager,
            game_object_manager: game_object_manager,
            world_manager: WorldManager::new(&mut BasicRoomGenerator::new(theme), &mut event_manager),
            input: InputHandler::new(),
            event_manager: event_manager
        }
    }
}

impl EventHandler for MyGame {
   fn update(&mut self, _context: &mut Context) -> GameResult {
        // Update code here...
        self.game_object_manager.update(&mut self.sprite_manager, &self.input, &self.world_manager, &mut self.event_manager);
        self.event_manager.update_events(&mut self.game_object_manager);


        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        
        let mut canvas = graphics::Canvas::from_frame(context, Color::BLACK);
        canvas.set_sampler(Sampler::nearest_clamp());
        self.world_manager.draw_world(&mut self.sprite_manager);

        self.sprite_manager.draw_buffer_to_canvas(&mut canvas);
        canvas.finish(context)?;

        return Ok(());
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