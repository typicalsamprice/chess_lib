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

use crate::bitboard::Bitboard;
use std::ops::Not;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl Color {
    pub fn pawn_push(self) -> Box<fn(Bitboard) -> Bitboard> {
        Box::new(match self {
            Self::White => push_n,
            Self::Black => push_s,
        })
    }
}

fn push_n(b: Bitboard) -> Bitboard {
    b << 8
}
fn push_s(b: Bitboard) -> Bitboard {
    b >> 8
}

impl Not for Color {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
