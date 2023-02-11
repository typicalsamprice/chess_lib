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

use crate::color::Color;
use std::fmt;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Piece(u8);

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PType {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub const NULL: Self = Self(0xF);
    #[inline]
    pub const fn new(ty: PType, color: Color) -> Self {
        Self(((color as u8) << 3) | ty as u8)
    }
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 != 0xF
    }

    #[inline]
    pub const fn inner(self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn color(self) -> Color {
        match self.0 & 0b1000 {
            0b1000 => Color::Black,
            0 => Color::White,
            _ => panic!(),
        }
    }
    #[inline]
    pub const fn kind(self) -> PType {
        unsafe { std::mem::transmute(self.0 & 7) }
    }
}

impl PType {
    pub fn is_slider(self) -> bool {
        self >= Self::Bishop && self <= Self::Queen
    }

    pub const fn valuef(self) -> f64 {
        match self {
            Self::Pawn => 1.0,
            Self::Knight => 3.0,
            Self::Bishop => 3.0,
            Self::Rook => 5.0,
            Self::Queen => 9.0,
            Self::King => 0.0
        }
    }
}

impl TryFrom<char> for PType {
    type Error = ();
    fn try_from(c: char) -> Result<Self, ()> {
        Ok(match c {
            'p' => Self::Pawn,
            'n' => Self::Knight,
            'b' => Self::Bishop,
            'r' => Self::Rook,
            'q' => Self::Queen,
            'k' => Self::King,
            _ => return Err(()),
        })
    }
}
impl TryFrom<char> for Piece {
    type Error = ();
    fn try_from(c: char) -> Result<Self, ()> {
        let ty = PType::try_from(c.to_ascii_lowercase())?;
        if c.is_ascii_uppercase() {
            Ok(Self::new(ty, Color::White))
        } else {
            Ok(Self::new(ty, Color::Black))
        }
    }
}

impl From<PType> for char {
    fn from(ty: PType) -> Self {
        let i = ty as usize;
        b"pnbrqk"[i] as Self
    }
}
impl From<Piece> for char {
    fn from(p: Piece) -> Self {
        if !p.is_ok() {
            return ' ';
        }
        let ty = char::from(p.kind());
        if p.color() == Color::White {
            ty.to_ascii_uppercase()
        } else {
            ty
        }
    }
}

impl fmt::Display for PType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", b"pnbrqk"[*self as usize] as char)
    }
}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = self.kind().to_string().chars().next().unwrap();
        let ch = if self.color() == Color::White {
            c.to_ascii_uppercase()
        } else {
            c
        };
        write!(f, "{ch}")
    }
}
