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

use std::ops::{BitXor, BitXorAssign};

use bitintr::Popcnt;

use crate::prng::Prng;
use crate::square::Square;
use crate::{filerank::File, piece::PType, prelude::Color};

#[derive(Debug, Default, Clone, Copy)]
pub struct Key(pub u64);

impl BitXor for Key {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}
impl BitXorAssign for Key {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

type ZA<const N: usize> = [Key; N];
type ZA2<const N: usize, const M: usize> = [ZA<N>; M];

static mut Z_PAWNS: ZA2<2, 64> = [[Key(0); 2]; 64];
static mut Z_PT: ZA2<6, 64> = [[Key(0); 6]; 64];
static mut Z_COL: Key = Key(0);
static mut Z_EP_FILE: ZA<8> = [Key(0); 8];
static mut Z_CASTLE: ZA<4> = [Key(0); 4];
static mut Z_NPAWNS: Key = Key(0);

impl Key {
    pub const fn new(seed: u64) -> Self {
        Self(
            seed.wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407),
        )
    }
    pub fn rand(prng: &mut Prng) -> Self {
        Self::new(prng.sample())
    }
}

pub(crate) unsafe fn init_zobrist() {
    let prng = &mut Prng::new(0x1af4342bd258);
    Z_COL = Key::rand(prng);
    Z_NPAWNS = Key::rand(prng);
    for i in 0..64 {
        for j in 0..2 {
            Z_PAWNS[i][j] = Key::rand(prng);
        }
        for j in 0..6 {
            Z_PT[i][j] = Key::rand(prng);
        }
    }
    for zep in &mut Z_EP_FILE {
        *zep = Key::rand(prng);
    }
    for zc in &mut Z_CASTLE {
        *zc = Key::rand(prng);
    }
}

pub fn color() -> Key {
    unsafe { Z_COL }
}
pub fn no_pawns() -> Key {
    unsafe { Z_NPAWNS }
}
pub fn ep_file(f: File) -> Key {
    unsafe { Z_EP_FILE[f as usize] }
}
pub fn piece(ty: PType, s: Square) -> Key {
    unsafe { Z_PT[s.inner() as usize][ty as usize] }
}
pub fn pawn(color: Color, s: Square) -> Key {
    unsafe { Z_PAWNS[s.inner() as usize][color as usize] }
}
pub fn castle(bit: u8) -> Option<Key> {
    if bit.popcnt() != 1 {
        return None;
    }
    unsafe { Some(Z_CASTLE[bit.ilog2() as usize]) }
}
