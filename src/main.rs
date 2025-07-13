
#![allow(dead_code)] // this line make cargo shut the fuck up
#![allow(unused_variables)]

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

use crate::engine::objects::object_summon::ObjectSummonRegistration;


fn main() {
    // ================================
    // THE LIST
    // ================================
    
    // [ ] - to be done
    // [a] - assigned
    // [d] - doing right now
    // [x] - done
    // [p] - postponed
    // [-] - cancelled

    // ================================

    // 1 [ ] Gamblig
    //   1.1 [ ] Level Randomness system
    //       1.1.1 [ ] Random big enemies
    //       1.1.2 [ ] Random items
    //       1.1.3 [ ] Random pickups
    // 2 [ ] Items
    //   2.1 [ ] Weapons
    //   2.2 [ ] Pickups
    //   2.3 [ ] Money?
    //   2.4 [ ] Crafting materials?
    //   2.5 [ ] Armor
    // 3 [ ] Levels
    //   3.1 [x] Generation
    //   3.2 [ ] Locked rooms
    //       3.2.1 [ ] Button rooms
    //       3.2.2 [ ] Key rooms
    //   3.3 [ ] Level progression
    //       3.3.1 [ ] Level exit
    //       3.3.2 [ ] Generating next level
    //       3.3.3 [ ] Special levels
    //   3.4 [ ] Randomize graphics
    // 4 [ ] Engine
    //   4.1 [ ] Global values
    //   4.2 [ ] Ui
    //   4.3 [ ] Sounds
    // 5 [ ] Game content
    // TODO : fill this in
    // 6 [ ] Game loop
    //   6.1 [ ] Enemies
    //       6.1.1 [x] Enemy awearness
    //       6.1.2 [a] Enemy "pathfinding"
    //   6.2 [a] Weapons

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
        .window_mode( WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::True).dimensions(1920.0, 1080.0) )
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
        context.gfx.set_fullscreen(ggez::conf::FullscreenType::True).unwrap();
        
        // sprite manager
        let mut sprite_manager = DrawingManager::new(context, Vec::from_iter(DrawingLayer::VALUES.iter().map(|it|{return it.get_value()})));
        sprite_manager.set_camera_zoom(5.0);
        // game object manager
        let mut game_object_manager = GameObjectManager::new();

        for summon_registration in inventory::iter::<ObjectSummonRegistration> {
            game_object_manager.register_summon(summon_registration);
        }

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
   fn update(&mut self, context: &mut Context) -> GameResult {
 
        let delta = (context.time.delta().as_millis() as f32) / 16.6666;
        // Update code here...
        self.game_object_manager.update(&mut self.sprite_manager, &self.input, &self.world_manager, &mut self.event_manager, delta);
        self.event_manager.update_events(&mut self.game_object_manager);


        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        
        let mut canvas = graphics::Canvas::from_frame(context, Color::BLACK);
        canvas.set_sampler(Sampler::nearest_clamp());
        self.world_manager.draw_world(&mut self.sprite_manager);
        self.game_object_manager.draw_objects(&mut self.sprite_manager);

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