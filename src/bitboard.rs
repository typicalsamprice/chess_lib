use std::ops;

use crate::square::Square;

#[cfg(feature = "pext")]
use bitintr::Pext;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
    pub const fn inner(self) -> u64 {
        self.0
    }

    pub const fn more_than_one(self) -> bool {
        (self.0 - 1) & self.0 > 0
    }

    pub const fn popcnt(self) -> u32 {
        self.0.count_ones()
    }

    pub const fn zero(self) -> bool {
        self.0 == 0
    }
    pub const fn nonzero(self) -> bool {
        !self.zero()
    }

    pub fn and_not<T: Into<Self>>(self, rhs: T) -> Self {
        self &! rhs.into()
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
    where Self: ops::BitOr<T, Output = Self>
{
    fn bitor_assign(&mut self, rhs: T) {
        *self = *self | rhs;
    }
}
impl<T> ops::BitAndAssign<T> for Bitboard
    where Self: ops::BitAnd<T, Output = Self>
{
    fn bitand_assign(&mut self, rhs: T) {
        *self = *self & rhs;
    }
}
impl<T> ops::BitXorAssign<T> for Bitboard
    where Self: ops::BitXor<T, Output = Self>
{
    fn bitxor_assign(&mut self, rhs: T) {
        *self = *self ^ rhs;
    }
}
