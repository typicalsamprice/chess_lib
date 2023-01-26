use std::ops::Add;

use crate::square::Square;
use crate::color::Color;

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
        unsafe { std::mem::transmute(idx as u8) }
    }
}
impl From<u8> for Rank {
    fn from(idx: u8) -> Self {
        debug_assert!(idx <= Rank::Eight as u8);
        unsafe { std::mem::transmute(idx as u8) }
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
