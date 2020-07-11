use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;

pub struct Game {
    game_state: GameState
}

enum GameState {
    Playing
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        let game_state = GameState::Playing;

        Game {
          game_state
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        graphics::present(ctx)
    }
}

