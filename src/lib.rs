mod bitboard;
mod color;
mod filerank;
mod square;

mod init;
mod magic;
mod prng;

pub mod prelude {
    pub use crate::bitboard::Bitboard;
    pub use crate::filerank::*;
    pub use crate::square::Square;
    pub use crate::color::Color;
    pub use crate::init::{init as init_comp, pawn_attack, knight_attack, king_attack, between};
    pub use crate::magic::{initalize_magics, rook_moves, bishop_moves};
}

// If we want to use PEXT instructions
// Sometimes we don't even if it's available because it
// can be slow
#[cfg(feature = "pext")]
pub const USE_PEXT: bool = true;
#[cfg(not(feature = "pext"))]
pub const USE_PEXT: bool = false;

#[cfg(target_pointer_width = "64")]
pub const IS_64_BIT: bool = true;
#[cfg(not(target_pointer_width = "64"))]
pub const IS_64_BIT: bool = false;