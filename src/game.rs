use ggez::{
    Context, GameResult, event,
    glam::*,
    graphics::{Canvas, Color, DrawMode, Mesh, MeshBuilder, Rect},
};

use crate::logic::{Pieces, StateChange};

pub struct GameState {
    board_mesh: Mesh,
    hitcircles_mesh: Mesh,
    drawing_hitcircles: bool,
    pieces: Pieces,
    pieces_mesh: Mesh,
}

impl GameState {
    /// A mesh that draws the tiles of a board.
    ///
    /// If errors don't happen, the output should be a constant.
    fn board_mesh(ctx: &Context) -> GameResult<Mesh> {
        let mut mb = MeshBuilder::new();

        let mut top = 0;
        let mut left = 1;
        let mut next_row_immediate_dark = true;

        const NUM_TILES: u8 = 8 * 8;
        const NUM_DARK_TILES: u8 = NUM_TILES / 2;

        for _ in 0..NUM_DARK_TILES {
            mb.rectangle(
                DrawMode::fill(),
                Rect::new_i32(100 * left, 100 * top, 100, 100),
                Color::from_rgb(181, 136, 99),
            )?;

            left += 2;
            if left >= 8 {
                left = if next_row_immediate_dark { 0 } else { 1 };
                next_row_immediate_dark = !next_row_immediate_dark;
                top += 1;
            }
        }
        Ok(Mesh::from_data(ctx, mb.build()))
    }

    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let board_mesh = Self::board_mesh(ctx)?;
        let hitcircles_mesh = Pieces::filled().get_mesh(ctx)?;
        let drawing_hitcircles = false;
        let pieces = Pieces::default();
        let pieces_mesh = pieces.get_mesh(ctx)?;

        Ok(GameState {
            board_mesh,
            hitcircles_mesh,
            drawing_hitcircles,
            pieces,
            pieces_mesh,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        for state_change in self.pieces.handle_click(x, y).unwrap_or(vec![]) {
            match state_change {
                StateChange::Deselected => self.drawing_hitcircles = false,
                StateChange::Selected => self.drawing_hitcircles = true,
                StateChange::PieceMoved => self.pieces_mesh = self.pieces.get_mesh(ctx)?,
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(240, 217, 181));

        canvas.draw(&self.board_mesh, Vec2::ZERO);
        canvas.draw(&self.pieces_mesh, Vec2::ZERO);
        if self.drawing_hitcircles {
            canvas.draw(&self.hitcircles_mesh, Vec2::ZERO);
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}
