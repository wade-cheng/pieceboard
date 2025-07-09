use ggez::{
    GameResult,
    conf::{WindowMode, WindowSetup},
    event,
};
use pieceboard::constants::BOARD_PX;
use pieceboard::game::GameState;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(WindowMode::default().dimensions(BOARD_PX, BOARD_PX))
        .window_setup(WindowSetup::default().title("movable pieces on board"));

    let (mut ctx, event_loop) = cb.build()?;

    let state = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}
