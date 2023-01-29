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

use bitintr::{Pext, Popcnt};

use crate::bitboard::Bitboard;
use crate::filerank::{File, Rank};
use crate::prng::Prng;
use crate::square::Square;
use crate::{IS_64_BIT, USE_PEXT};

static mut BISHOP_ATTACK_TABLE: [Bitboard; 0x1480] = [Bitboard::ZERO; 0x1480];
static mut ROOK_ATTACK_TABLE: [Bitboard; 0x19000] = [Bitboard::ZERO; 0x19000];

static mut B_MAGICS: [Magic; 64] = [Magic::nulled(); 64];
static mut R_MAGICS: [Magic; 64] = [Magic::nulled(); 64];

#[derive(Debug, Clone, Copy)]
struct Magic {
    mask: u64,
    magic: u64,
    shift: u32,
    ptr: usize,
}

impl Magic {
    #[inline]
    const fn nulled() -> Self {
        Self {
            mask: 0,
            magic: 0,
            shift: 0,
            ptr: 0,
        }
    }

    fn index(&self, occ: Bitboard) -> u32 {
        if USE_PEXT {
            return occ.inner().pext(self.mask) as u32;
        }

        if IS_64_BIT {
            return (((occ & Bitboard::new(self.mask))
                .inner()
                .wrapping_mul(self.magic))
                >> self.shift) as u32;
        }

        let lo = (occ.inner() as u32) & (self.mask as u32);
        let hi = ((occ.inner() >> 32) as u32) & ((self.mask >> 32) as u32);

        (lo * (self.magic as u32) ^ (hi * ((self.magic >> 32) as u32))) >> self.shift
    }
}

fn slider_attack<const IS_ROOK: bool>(square: Square, occupied: Bitboard) -> Bitboard {
    let mut attack = Bitboard::ZERO;

    let shifts = if IS_ROOK {
        [8, -8, 1, -1]
    } else {
        [7, -7, 9, -9]
    };
    let mut s;
    for shift in shifts {
        s = square;
        while s.safe_move(shift) && (occupied & s).zero() {
            s = unsafe { Square::new((s.inner() as i32 + shift) as u8) };
            attack |= s;
        }
    }

    attack
}

fn init_magics<const IS_ROOK: bool>(
    attack_table: &'static mut [Bitboard],
    magic_table: &'static mut [Magic],
) {
    let mut sz = 0;
    let mut b = Bitboard::ZERO;
    let mut occ: [Bitboard; 4096] = [Bitboard::ZERO; 4096];
    let mut refer: [Bitboard; 4096] = [Bitboard::ZERO; 4096];

    let seeds = [
        // 64-bit seeds
        [8977, 44560, 545343, 38998, 5731, 95205, 104912, 17020],
        // 32-bit seeds
        [728, 10316, 55013, 32803, 12281, 15100, 16645, 255],
    ];

    Bitboard::MAX.map_by_square(|s| {
        let ptr = if s.inner() == 0 {
            0
        } else {
            magic_table[s.inner() as usize - 1].ptr + sz
        };
        let edges = (Bitboard::from(File::A) | Bitboard::from(File::H)).and_not(s.file())
            | (Bitboard::from(Rank::One) | Bitboard::from(Rank::Eight)).and_not(s.rank());

        let m = &mut magic_table[s.inner() as usize];
        m.mask = slider_attack::<IS_ROOK>(s, Bitboard::ZERO).inner() & !edges.inner();
        let max: u32 = if IS_64_BIT { 64 } else { 32 };
        m.shift = max.abs_diff(m.mask.popcnt() as u32);
        m.ptr = ptr;

        b = Bitboard::ZERO;
        sz = 0;

        loop {
            occ[sz] = b;
            refer[sz] = slider_attack::<IS_ROOK>(s, b);

            if USE_PEXT {
                attack_table[b.inner().pext(m.mask) as usize] = refer[sz];
            }

            sz += 1;
            b = Bitboard::new(b.inner().wrapping_sub(m.mask) & m.mask);

            if b.zero() {
                break;
            }
        }

        if USE_PEXT {
            return;
        }

        let mut cnt = 0;
        let mut epoch = [0i32; 4096];
        let mut i = 0;

        let mut rng = Prng::new(seeds[IS_64_BIT as usize][s.rank() as usize]);

        while i < sz {
            m.magic = 0; // Just reset it.
            while (m.magic.wrapping_mul(m.mask) >> 56).popcnt() < 6 {
                m.magic = rng.sparse();
            }

            cnt += 1;
            i = 0;

            while i < sz {
                let idx = m.index(occ[i]) as usize;
                if epoch[idx] < cnt {
                    epoch[idx] = cnt;
                    attack_table[m.ptr + idx] = refer[i];
                } else if attack_table[m.ptr + idx] != refer[i] {
                    break;
                }

                i += 1;
            }
        }
    });
}

pub fn initalize_magics() {
    unsafe {
        init_magics::<true>(&mut ROOK_ATTACK_TABLE, &mut R_MAGICS);
        init_magics::<false>(&mut BISHOP_ATTACK_TABLE, &mut B_MAGICS);
    }
}

pub fn rook_moves(square: Square, occ: Bitboard) -> Bitboard {
    let magic = unsafe { R_MAGICS[square.inner() as usize] };
    let idx = magic.index(occ);

    unsafe { ROOK_ATTACK_TABLE[magic.ptr + idx as usize] }
}
pub fn bishop_moves(square: Square, occ: Bitboard) -> Bitboard {
    let magic = unsafe { B_MAGICS[square.inner() as usize] };
    let idx = magic.index(occ);

    unsafe { BISHOP_ATTACK_TABLE[magic.ptr + idx as usize] }
}
