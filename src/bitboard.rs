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
use std::ops;

use crate::prelude::{File, Rank};
use crate::prelude::Square;
use crate::prelude::Color;

#[cfg(feature = "pext")]
use bitintr::Pext;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const ZERO: Self = Self(0);
    pub const MAX: Self = Self(!0);

    #[inline(always)]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
    #[inline(always)]
    pub const fn inner(self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub const fn more_than_one(self) -> bool {
        (self.0 - 1) & self.0 > 0
    }

    #[inline(always)]
    pub const fn zero(self) -> bool {
        self.0 == 0
    }
    #[inline(always)]
    pub const fn nonzero(self) -> bool {
        !self.zero()
    }

    #[inline(always)]
    pub const fn popcnt(self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn and_not<T: Into<Self>>(self, rhs: T) -> Self {
        self & !rhs.into()
    }

    #[inline(always)]
    pub const fn const_or(self, rhs: Self) -> Self {
        Self(self.inner() | rhs.inner())
    }
    #[inline(always)]
    pub const fn const_and(self, rhs: Self) -> Self {
        Self(self.inner() & rhs.inner())
    }
    #[inline(always)]
    pub const fn const_xor(self, rhs: Self) -> Self {
        Self(self.inner() ^ rhs.inner())
    }

    pub fn map_by_square<F: FnMut(Square)>(self, mut f: F) {
        let mut copy = self;
        while copy.nonzero() {
            f(copy.pop_square());
        }
    }
    pub fn map_by_board<F: FnMut(Self)>(self, mut f: F) {
        let mut copy = self;
        loop {
            if copy.zero() {
                return;
            }

            let first_one = copy & -copy;
            copy.0 &= copy.0 - 1;
            f(first_one);
        }
    }

    #[inline(always)]
    pub const fn get_square(self) -> Square {
        unsafe { Square::new(self.0.trailing_zeros() as u8) }
    }

    #[inline(always)]
    pub fn pop_square(&mut self) -> Square {
        let s = self.get_square();
        self.0 &= self.0 - 1;
        s
    }

    #[inline(always)]
    pub const fn relative(self, color: Color) -> Self {
        match color {
            Color::White => self,
            Color::Black => Self(self.0.swap_bytes()),
        }
    }
}

pub const FILE_BB: [Bitboard; 8] = [
    Bitboard(0x0101010101010101),
    Bitboard(0x0101010101010101 << 1),
    Bitboard(0x0101010101010101 << 2),
    Bitboard(0x0101010101010101 << 3),
    Bitboard(0x0101010101010101 << 4),
    Bitboard(0x0101010101010101 << 5),
    Bitboard(0x0101010101010101 << 6),
    Bitboard(0x0101010101010101 << 7),
];
pub const RANK_BB: [Bitboard; 8] = [
    Bitboard(0xFF),
    Bitboard(0xFF << 8),
    Bitboard(0xFF << (2 * 8)),
    Bitboard(0xFF << (3 * 8)),
    Bitboard(0xFF << (4 * 8)),
    Bitboard(0xFF << (5 * 8)),
    Bitboard(0xFF << (6 * 8)),
    Bitboard(0xFF << (7 * 8)),
];

// Crate-visible methods
impl Bitboard {
    pub(crate) const fn arr<const DIM: usize>() -> [Self; DIM] {
        [Self::ZERO; DIM]
    }
    pub(crate) const fn arr_2d<const DIM_1: usize, const DIM_2: usize>() -> [[Self; DIM_1]; DIM_2] {
        [[Self::ZERO; DIM_1]; DIM_2]
    }
}

#[cfg(feature = "pext")]
impl Pext for Bitboard {
    fn pext(self, other: Self) -> Self {
        Self(self.0.pext(other.0))
    }
}

impl From<Square> for Bitboard {
    fn from(square: Square) -> Self {
        Self(1 << square.inner())
    }
}
impl From<File> for Bitboard {
    fn from(f: File) -> Self {
        FILE_BB[f as usize]
    }
}
impl From<Rank> for Bitboard {
    fn from(r: Rank) -> Self {
        RANK_BB[r as usize]
    }
}

impl ops::Shl<u32> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: u32) -> Self {
        Self(self.0 << rhs)
    }
}
impl ops::Shr<u32> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: u32) -> Self {
        Self(self.0 >> rhs)
    }
}

impl<T> ops::ShlAssign<T> for Bitboard
where
    Self: ops::Shl<T, Output = Self>,
{
    fn shl_assign(&mut self, rhs: T) {
        *self = *self << rhs;
    }
}
impl<T> ops::ShrAssign<T> for Bitboard
where
    Self: ops::Shr<T, Output = Self>,
{
    fn shr_assign(&mut self, rhs: T) {
        *self = *self >> rhs;
    }
}

impl ops::Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self {
        Self(!self.0)
    }
}
impl ops::Neg for Bitboard {
    type Output = Self;
    fn neg(self) -> Self {
        Self(self.0.wrapping_neg())
    }
}

impl<T: Into<Self>> ops::BitOr<T> for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: T) -> Self {
        Self(self.0 | rhs.into().0)
    }
}
impl<T: Into<Self>> ops::BitAnd<T> for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: T) -> Self {
        Self(self.0 & rhs.into().0)
    }
}
impl<T: Into<Self>> ops::BitXor<T> for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: T) -> Self {
        Self(self.0 ^ rhs.into().0)
    }
}

impl<T> ops::BitOrAssign<T> for Bitboard
where
    Self: ops::BitOr<T, Output = Self>,
{
    fn bitor_assign(&mut self, rhs: T) {
        *self = *self | rhs;
    }
}
impl<T> ops::BitAndAssign<T> for Bitboard
where
    Self: ops::BitAnd<T, Output = Self>,
{
    fn bitand_assign(&mut self, rhs: T) {
        *self = *self & rhs;
    }
}
impl<T> ops::BitXorAssign<T> for Bitboard
where
    Self: ops::BitXor<T, Output = Self>,
{
    fn bitxor_assign(&mut self, rhs: T) {
        *self = *self ^ rhs;
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::with_capacity(72);

        for i in 0..8 {
            for j in 0..8 {
                if self.0 & (1 << ((7 - i) * 8 + j)) > 0 {
                    s.push('1');
                } else {
                    s.push('.');
                }
                s.push(' ');
            }

            s.push('\n');
        }

        write!(f, "{s}")
    }
}
