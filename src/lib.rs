/*
    ChessLib, a UCI chess engine
    Copyright (C) 2023 Sam Price

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

mod bitboard;
mod chessmove;
mod color;
mod filerank;
mod init;
mod magic;
mod movegen;
mod piece;
mod prng;
mod square;

pub mod prelude {
    pub use crate::bitboard::*;
    pub use crate::color::*;
    pub use crate::filerank::*;
    pub use crate::init::{between, init as init_comp, king_attack, knight_attack, pawn_attack};
    pub use crate::magic::{bishop_moves, initalize_magics, rook_moves};
    pub use crate::piece::*;
    pub use crate::square::*;
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
