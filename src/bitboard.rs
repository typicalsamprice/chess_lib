use std::fmt;
use std::ops;

use crate::square::Square;
use crate::filerank::{File, Rank};

#[cfg(feature = "pext")]
use bitintr::Pext;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const ZERO: Self = Self(0);
    pub const MAX: Self = Self(!0);

    pub const fn new(value: u64) -> Self {
        Self(value)
    }
    pub const fn inner(self) -> u64 {
        self.0
    }

    pub const fn more_than_one(self) -> bool {
        (self.0 - 1) & self.0 > 0
    }

    pub const fn zero(self) -> bool {
        self.0 == 0
    }
    pub const fn nonzero(self) -> bool {
        !self.zero()
    }

    pub const fn popcnt(self) -> u32 {
        self.0.count_ones()
    }

    pub fn and_not<T: Into<Self>>(self, rhs: T) -> Self {
        self & !rhs.into()
    }

    pub fn map_by_square<F: FnMut(Square)>(self, mut f: F) {
        let mut copy = self;
        loop {
            if copy.zero() { return; }
            let tz = copy.0.trailing_zeros();
            debug_assert!(tz < 64);
            let lsb = unsafe { Square::new(tz as u8) };
            debug_assert!(lsb.is_ok());
            copy.0 &= copy.0 - 1;
            f(lsb);
        }
    }
    pub fn map_by_board<F: FnMut(Self)>(self, mut f: F) {
        let mut copy = self;
        loop {
            if copy.zero() { return; }

            let first_one = copy & -copy;
            copy.0 &= copy.0 - 1;
            f(first_one);
        }
    }

    pub const fn get_square(self) -> Square {
        if self.zero() { return Square::NULL }
        unsafe { Square::new(self.0.trailing_zeros() as u8) }
    }
}

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
        debug_assert!(square.is_ok());
        Self(1 << square.inner())
    }
}
impl From<File> for Bitboard {
    fn from(f: File) -> Self {
        Self(f.as_mask())
    }
}
impl From<Rank> for Bitboard {
    fn from(r: Rank) -> Self {
        Self(r.as_mask())
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
    where Self: ops::Shl<T, Output = Self>
{
    fn shl_assign(&mut self, rhs: T) {
        *self = *self << rhs;
    }
}
impl<T> ops::ShrAssign<T> for Bitboard
    where Self: ops::Shr<T, Output = Self>
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
            }

            s.push('\n');
        }

        write!(f, "{s}")
    }
}
