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

use crate::piece::PType;
use crate::square::Square;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Move(u32);

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum MType {
    #[default]
    Normal,
    EnPassant,
    Promotion,
    Castle,
}

impl Move {
    pub const NULL: Self = Self(0);

    #[inline]
    pub const fn is_ok(self) -> bool {
        self.from().inner() != self.to().inner()
    }

    #[inline]
    pub const fn from(self) -> Square {
        unsafe { Square::new(self.0 as u8 & 0x3f) }
    }
    #[inline]
    pub const fn to(self) -> Square {
        unsafe { Square::new(((self.0 >> 6) & 0x3f) as u8) }
    }
    #[inline]
    pub const fn kind(self) -> MType {
        unsafe { std::mem::transmute((self.0 >> 12) as u8 & 3) }
    }
    #[inline]
    pub const fn promo(self) -> PType {
        unsafe { std::mem::transmute((self.0 >> 14) as u8) }
    }

    #[inline]
    pub fn new(from: Square, to: Square) -> Self {
        let f6 = from.inner() as u32;
        let s6 = (to.inner() as u32) << 6;
        Self(f6 | s6)
    }

    #[inline]
    pub const fn add_type(self, ty: MType) -> Self {
        Self(self.0 | ((ty as u32) << 12))
    }
    #[inline]
    pub const fn add_promo(self, ty: PType) -> Self {
        Self(self.add_type(MType::Promotion).0 | ((ty as u32) << 14))
    }

    #[cfg(debug_assertions)]
    pub fn all_move_data(self) -> String {
        format!(
            "{}{} T: {:?} P: {:?}",
            self.from(),
            self.to(),
            self.kind(),
            self.promo()
        )
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prom = if self.is_ok() && self.kind() == MType::Promotion {
            char::from(self.promo()).to_string()
        } else {
            String::new()
        };
        write!(f, "{}{}{}", self.from(), self.to(), prom)
    }
}
