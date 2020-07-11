mod core;

use crate::core::game;
use ggez::{conf, ContextBuilder};
use ggez::event::{self};

fn main() {
     let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "tylerdotdev")
          .window_setup(conf::WindowSetup::default().title("My Game"))
		.build()
		.expect("Could not create ggez context.");

    let mut game = game::Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

