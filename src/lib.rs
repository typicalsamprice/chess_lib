mod bitboard;
mod color;
mod filerank;
mod square;

mod init;
mod magic;

pub mod prelude {
    pub use crate::bitboard::Bitboard;
    pub use crate::filerank::*;
    pub use crate::square::Square;
    pub use crate::color::Color;
    pub use crate::init::{pawn_attack, knight_attack, king_attack, between};
}

// If we want to use PEXT instructions
// Sometimes we don't even if it's available because it
// can be slow
#[cfg(feature = "pext")]
pub const USE_PEXT: bool = true;
#[cfg(not(feature = "pext"))]
pub const USE_PEXT: bool = false;
