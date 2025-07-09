use std::sync::OnceLock;

use ggez::{
    Context, GameResult,
    glam::Vec2,
    graphics::{Color, DrawMode, Mesh, MeshBuilder},
};

use crate::constants::{BOARD_PX, HITCIRCLE_RADIUS, TILE_PX};

/// A piece on the board.
///
/// The rank must be within `1..=8` and the file within `'a'..='h'`.
#[derive(Clone)]
struct Piece {
    color: Color,
    rank: u8,
    file: char,
}

impl Piece {
    /// x-coordinate of this piece.
    fn x(&self) -> f32 {
        // 0-indexed file
        let x_offset =
            u8::try_from(self.file).expect("The file should have been within bounds.") - b"a"[0];
        TILE_PX / 2. + f32::from(x_offset) * TILE_PX
    }

    /// y-coordinate of this piece.
    fn y(&self) -> f32 {
        // 0-indexed rank
        let y_offset = self.rank - 1;
        BOARD_PX - (TILE_PX / 2. + f32::from(y_offset) * TILE_PX)
    }

    /// Whether this piece collides with the pixel coordinates.
    fn collidepoint(&self, x: f32, y: f32) -> bool {
        Vec2::distance_squared(Vec2::new(self.x(), self.y()), Vec2::new(x, y))
            < HITCIRCLE_RADIUS.powi(2)
    }

    /// Whether these two pieces are on the same tile.
    fn same_tile(&self, other: &Self) -> bool {
        self.rank == other.rank && self.file == other.file
    }
}

pub enum StateChange {
    Deselected,
    Selected,
    PieceMoved,
}

pub struct Pieces {
    inner: Vec<Piece>,
    selected_idx: Option<usize>,
}

impl Default for Pieces {
    fn default() -> Self {
        let mut inner = vec![];
        for (color, rank) in [
            (Color::WHITE, 1),
            (Color::WHITE, 2),
            (Color::BLACK, 7),
            (Color::BLACK, 8),
        ] {
            for file in 'a'..='h' {
                inner.push(Piece { color, rank, file })
            }
        }

        Self {
            inner,
            selected_idx: None,
        }
    }
}

static FILLED: OnceLock<Pieces> = OnceLock::new();

impl Pieces {
    pub fn handle_click(&mut self, x: f32, y: f32) -> Option<Vec<StateChange>> {
        // println!("selidx: {:?}", self.selected_idx); // 24 IS TOP LEFT, 0 BOT LEFT
        match self.selected_idx {
            Some(src_piece_idx) => {
                let mut state_changes = vec![StateChange::Deselected];
                if let Some(dest_piece) = Self::movable_pos(x, y)
                    && !self.inner[src_piece_idx].same_tile(&dest_piece)
                {
                    // if the click at x, y can send our piece to some dest_piece,
                    // and the destination piece is not our selected piece, move it.
                    self.inner[src_piece_idx].rank = dest_piece.rank;
                    self.inner[src_piece_idx].file = dest_piece.file;
                    let moved_piece = self.inner[src_piece_idx].clone();
                    self.inner.retain(|piece| !piece.same_tile(&dest_piece));
                    self.inner.push(moved_piece);
                    state_changes.push(StateChange::PieceMoved);
                }
                self.selected_idx = None;
                return Some(state_changes);
            }
            None => {
                for (i, piece) in self.inner.iter().enumerate() {
                    if piece.collidepoint(x, y) {
                        self.selected_idx = Some(i);
                        return Some(vec![StateChange::Selected]);
                    }
                }

                None
            }
        }
    }

    /// Whether a click at (x, y) landed on a valid hitcircle to move to.
    ///
    /// Returns a piece representation of that hitcircle.
    ///
    /// This simply checks over a [`Self::filled`]. We avoid repeated allocations
    /// via a [`std::sync::OnceLock`].
    fn movable_pos(x: f32, y: f32) -> Option<Piece> {
        let piece_hitcircles = FILLED.get_or_init(|| Self::filled());
        for piece in &piece_hitcircles.inner {
            if piece.collidepoint(x, y) {
                return Some(piece.clone());
            }
        }

        None
    }

    /// A group of colored pieces filling the board.
    ///
    /// Useful for drawing hit circles.
    pub fn filled() -> Self {
        let mut inner = vec![];
        for rank in 1..=8 {
            for file in 'a'..='h' {
                inner.push(Piece {
                    color: Color::from_rgba(250, 250, 200, 80),
                    rank,
                    file,
                })
            }
        }

        Self {
            inner,
            selected_idx: None,
        }
    }

    pub fn get_mesh(&self, ctx: &Context) -> GameResult<Mesh> {
        let mut mb = MeshBuilder::new();

        for piece in &self.inner {
            mb.circle(
                DrawMode::fill(),
                Vec2::new(piece.x(), piece.y()),
                HITCIRCLE_RADIUS,
                1.,
                piece.color,
            )?;
        }

        Ok(Mesh::from_data(ctx, mb.build()))
    }
}
