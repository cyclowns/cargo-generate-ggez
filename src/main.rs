use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = /* ContextBuilder params */

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct MyGame {
    // Your state here...
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Draw code here...
    }
}
