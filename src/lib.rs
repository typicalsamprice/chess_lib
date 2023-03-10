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
pub mod diagnostics;
pub mod evaluate;
mod filerank;
mod init;
mod magic;
mod movegen;
mod moveorder;
mod piece;
mod position;
mod prng;
pub mod search;
mod square;
mod thread;
mod tt;
pub mod zobrist;

pub mod prelude {
    pub use crate::bitboard::*;
    pub use crate::chessmove::*;
    pub use crate::color::Color;
    pub use crate::filerank::*;
    pub use crate::init::{between, line, king_attack, pawn_attack, knight_attack};
    pub use crate::magic::{bishop_moves, queen_moves, rook_moves};
    pub use crate::movegen::*;
    pub use crate::piece::*;
    pub use crate::position::*;
    pub use crate::square::*;
    pub use crate::zobrist::{Key, self};
}

pub fn initialize() {
    magic::initalize_magics();
    init::init();
    unsafe {
        zobrist::init_zobrist();
    }
}

// If we want to use PEXT instructions
// Sometimes we don't even if it's available because it
// can be slow (Zen + Zen2 architectures specifically)
#[cfg(all(feature = "pext", target_feature = "bmi2"))]
pub const USE_PEXT: bool = true;
#[cfg(not(all(feature = "pext", target_feature = "bmi2")))]
pub const USE_PEXT: bool = false;

#[cfg(target_pointer_width = "64")]
pub const IS_64_BIT: bool = true;
#[cfg(not(target_pointer_width = "64"))]
pub const IS_64_BIT: bool = false;

pub const MAX_MOVES: usize = 256;
pub const MAX_PLY: usize = 246;
