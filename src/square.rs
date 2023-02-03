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

use crate::filerank::{File, Rank};
use crate::color::Color;

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

    pub fn relative(self, color: Color) -> Self {
        Self::create(self.file(), self.rank().relative(color))
    }

    pub const fn file(self) -> File {
        unsafe { std::mem::transmute(self.0 & 7) }
    }
    pub const fn rank(self) -> Rank {
        unsafe { std::mem::transmute((self.0 >> 3) & 7) }
    }

    pub const fn in_line(self, other: Self) -> bool {
        if self.inner() == other.inner() { return false; }
        // Uses a lot of `as u8` to make it `const`-ified
        if (self.file() as u8 == other.file() as u8) || (self.rank() as u8 == other.rank() as u8) {
            true
        } else {
            (self.file() as u8).abs_diff(other.file() as u8)
                == (self.rank() as u8).abs_diff(other.rank() as u8)
        }
    }

    // Assumes the second square is in the middle
    pub fn in_line2(self, other1: Self, other2: Self) -> bool {
        if self.in_line(other1) && self.in_line(other2) && other1.in_line(other2) {
            let cmp = self.partial_cmp(&other1).unwrap();
            if other1.partial_cmp(&other2) != Some(cmp) {
                return false;
            }


        }

        false
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
