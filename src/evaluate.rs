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

use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Score(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value(pub i32);

impl Score {
    pub const fn new(middlegame: u16, endgame: u16) -> Self {
        Self(middlegame as u32 | ((endgame as u32) << 16))
    }

    pub const fn mg(self) -> u16 {
        self.0 as u16
    }
    pub const fn eg(self) -> u16 {
        (self.0 >> 16) as u16
    }
}

impl Add for Score {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub for Score {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl SubAssign for Score {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

type SA<const N: usize> = [Score; N];
macro_rules! S {
    ($MG:literal, $EG:literal) => { Score::new($MG, $EG) }
}

const NOT_KING_DEFENDER: SA<2> = [S!(9, 9), S!(7, 9)];

const CLEAN_OUTPOST: Score = Score::new(0, 10);
