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

use std::fmt;

use crate::color::Color;
use crate::filerank::{File, Rank};
use crate::init::between;
use crate::init::line;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Square(u8);

impl Square {
    pub const NULL: Self = Self(64);

    pub const fn is_ok(self) -> bool {
        self.0 < 64
    }
    pub const unsafe fn new(value: u8) -> Self {
        // Unsafe because this can create an invalid square
        Self(value)
    }
    pub const fn create(file: File, rank: Rank) -> Self {
        unsafe { Self::new(file as u8 + 8 * rank as u8) }
    }
    pub const fn inner(self) -> u8 {
        self.0
    }

    pub const fn relative(self, color: Color) -> Self {
        match color {
            Color::White => self,
            Color::Black => Self(self.0 ^ 56)
        }
    }

    // Squash it into the relative file left side
    pub fn weight_map_idx(self) -> usize {
        // To make sure that we can "see" the table upside down
        // and the squares are correctly mapped
        let flip_vert = self.0 as usize ^ 56;

        let correct_square = if self.file() >= File::E {
            flip_vert ^ 7
        } else {
            flip_vert
        };

        // Now this has to be convert to a 0..31 index
        // instead of the usual 0..63 index
        let f = correct_square & 7;
        let r = correct_square >> 3;
        4 * r + f
    }

    pub const fn file(self) -> File {
        unsafe { std::mem::transmute(self.0 & 7) }
    }
    pub const fn rank(self) -> Rank {
        unsafe { std::mem::transmute((self.0 >> 3) & 7) }
    }

    pub fn in_line(self, other: Self) -> bool {
        line(self, other).nonzero()
    }

    pub fn in_line2(self, other1: Self, other2: Self) -> bool {
        (line(self, other1) & other2).nonzero()
    }

    pub fn dist(self, other: Self) -> u32 {
        // Unsigned distance (king moves)
        let fd = (self.file() as u32).abs_diff(other.file() as u32);
        let rd = (self.rank() as u32).abs_diff(other.rank() as u32);
        fd.max(rd)
    }

    pub fn safe_move(self, jmp: i32) -> bool {
        if jmp < 0 && -jmp > self.inner() as i32 {
            return false;
        }
        let to = Self((self.inner() as i32 + jmp) as u8);
        to.is_ok() && self.dist(to) <= 2
    }
}

impl Default for Square {
    fn default() -> Self {
        Self(64)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", char::from(self.file()), char::from(self.rank()))
    }
}

pub mod individual_squares {
    use super::Square;
    macro_rules! const_make_square_from_chars {
        ($STR_SLICE:expr) => {{
            let f = unsafe { std::mem::transmute($STR_SLICE.as_bytes()[0] - b'A') };
            let r = unsafe { std::mem::transmute($STR_SLICE.as_bytes()[1] - b'1') };
            Square::create(f, r)
        }};
    }

    macro_rules! make_pub_const {
        ($($SQ:ident),+) => {
            $(
                pub const $SQ: Square = const_make_square_from_chars!(stringify!($SQ));
            )+
        };
    }

    make_pub_const!(
        A1, B1, C1, D1, E1, F1, G1, H1, A2, B2, C2, D2, E2, F2, G2, H2, A3, B3, C3, D3, E3, F3, G3,
        H3, A4, B4, C4, D4, E4, F4, G4, H4, A5, B5, C5, D5, E5, F5, G5, H5, A6, B6, C6, D6, E6, F6,
        G6, H6, A7, B7, C7, D7, E7, F7, G7, H7, A8, B8, C8, D8, E8, F8, G8, H8
    );
}
