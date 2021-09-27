use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::filesystem;
use ggez::conf::WindowMode;

//const ICON: &'static str = "resources/wN.png";

fn main() {
    // Make a Context.
    let window_mode = WindowMode::default()
        .dimensions(1920.0, 2000.0);

    let (mut context, event_loop) = ContextBuilder::new("Chess", "Olle Jernström")
        .window_mode(window_mode)
        .build()
        .expect("Could not create ggez context!");
    graphics::set_window_title(&context, "Tjack!");
    /*
    let result = graphics::set_window_icon(&mut context, Some(ICON));
    
    match result {
        Ok(()) => (),
        Err(err) => println!("ICON NOT LOADED: {}", err),
    }
    */
    
    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut context);

    // Run!
    event::run(context, event_loop, my_game);
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_context: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler<ggez::GameError> for MyGame {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, Color::WHITE);
        // Draw code here...
        graphics::present(context)
    }
}