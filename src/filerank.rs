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

use std::ops::Add;

use crate::color::Color;
use crate::square::Square;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl File {
    pub const fn as_mask(self) -> u64 {
        0x0101010101010101 << self as u8
    }
}

impl Rank {
    pub const fn as_mask(self) -> u64 {
        0xFF << (8 * self as u8)
    }

    pub fn relative(self, color: Color) -> Self {
        Self::from(self as u8 + (color as u8 * (7 - 2 * self as u8)))
    }
}

impl From<u8> for File {
    fn from(idx: u8) -> Self {
        debug_assert!(idx <= File::H as u8);
        unsafe { std::mem::transmute(idx) }
    }
}
impl From<u8> for Rank {
    fn from(idx: u8) -> Self {
        debug_assert!(idx <= Rank::Eight as u8);
        unsafe { std::mem::transmute(idx) }
    }
}

impl TryFrom<char> for File {
    type Error = ();
    fn try_from(c: char) -> Result<Self, ()> {
        if !('a'..='h').contains(&c) {
            return Err(());
        }

        Ok(Self::from(c as u8 - b'a'))
    }
}
impl TryFrom<char> for Rank {
    type Error = ();
    fn try_from(c: char) -> Result<Self, ()> {
        if !('1'..='8').contains(&c) {
            return Err(());
        }

        Ok(Self::from(c as u8 - b'1'))
    }
}

impl Add<File> for Rank {
    type Output = Square;
    fn add(self, rhs: File) -> Square {
        Square::create(rhs, self)
    }
}
impl Add<Rank> for File {
    type Output = Square;
    fn add(self, rhs: Rank) -> Square {
        Square::create(self, rhs)
    }
}
