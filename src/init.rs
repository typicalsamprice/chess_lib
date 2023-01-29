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

use std::cmp::Ordering;

use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::filerank::File;
use crate::square::Square;

static mut KNIGHT_ATTACKS: [Bitboard; 64] = Bitboard::arr::<64>();
static mut KING_ATTACKS: [Bitboard; 64] = Bitboard::arr::<64>();
static mut PAWN_ATTACKS: [[Bitboard; 2]; 64] = Bitboard::arr_2d::<2, 64>();
static mut BETWEEN_SQUARES: [[Bitboard; 64]; 64] = Bitboard::arr_2d::<64, 64>();

pub fn init() {
    init_pawn_attacks();
    init_knight_attacks();
    init_king_attacks();
    init_between_square_lines();
}

pub fn knight_attack(square: Square) -> Bitboard {
    unsafe { KNIGHT_ATTACKS[square.inner() as usize] }
}
pub fn king_attack(square: Square) -> Bitboard {
    unsafe { KING_ATTACKS[square.inner() as usize] }
}
pub fn pawn_attack(square: Square, color: Color) -> Bitboard {
    unsafe { PAWN_ATTACKS[square.inner() as usize][color as usize] }
}
pub fn between(s1: Square, s2: Square) -> Bitboard {
    unsafe { BETWEEN_SQUARES[s1.inner() as usize][s2.inner() as usize] }
}

fn init_pawn_attacks() {
    Bitboard::MAX.map_by_board(|square| {
        // Calculate this stuff;
        let s = square.get_square().inner() as usize;
        let horiz = (square << 1).and_not(File::A) | (square >> 1).and_not(File::H);
        unsafe {
            PAWN_ATTACKS[s] = [horiz << 8, horiz >> 8];
        }
    });
}
fn init_knight_attacks() {
    let shift_w = |bb: Bitboard| (bb >> 1).and_not(File::H);
    let shift_e = |bb: Bitboard| (bb << 1).and_not(File::A);

    Bitboard::MAX.map_by_board(|square| {
        let nnw = shift_w(square) << 16;
        let nne = shift_e(square) << 16;
        let nww = shift_w(shift_w(square)) << 8;
        let nee = shift_e(shift_e(square)) << 8;

        let ssw = shift_w(square) >> 16;
        let sse = shift_e(square) >> 16;
        let sww = shift_w(shift_w(square)) >> 8;
        let see = shift_e(shift_e(square)) >> 8;

        unsafe {
            KNIGHT_ATTACKS[square.get_square().inner() as usize] =
                nnw | nne | nww | nee | ssw | sse | sww | see;
        }
    });
}
fn init_king_attacks() {
    Bitboard::MAX.map_by_board(|square| {
        let diags = unsafe { PAWN_ATTACKS[square.get_square().inner() as usize] };
        let vert = (square << 8) | (square >> 8);
        let horz = (square << 1).and_not(File::A) | (square >> 1).and_not(File::H);

        unsafe {
            KING_ATTACKS[square.get_square().inner() as usize] = diags[0] | diags[1] | vert | horz;
        }
    });
}
fn init_between_square_lines() {
    Bitboard::MAX.map_by_board(|sq1b| {
        Bitboard::MAX.map_by_board(|sq2b| {
            let s1 = sq1b.get_square();
            let s2 = sq2b.get_square();

            if s1 == s2
                || !s1.in_line(s2)
                || unsafe { BETWEEN_SQUARES[s1.inner() as usize][s2.inner() as usize].nonzero() }
            {
                return;
            }

            let shift_hz = match s1.file().partial_cmp(&s2.file()) {
                Some(Ordering::Less) => 1,
                Some(Ordering::Greater) => -1,
                Some(Ordering::Equal) => 0,
                None => panic!(),
            };
            let shift_vt = match s1.rank().partial_cmp(&s2.rank()) {
                Some(Ordering::Less) => 8,
                Some(Ordering::Greater) => -8,
                Some(Ordering::Equal) => 0,
                None => panic!(),
            };

            let total_shift: i32 = shift_hz + shift_vt;
            debug_assert_ne!(total_shift, 0);

            let shift_left = total_shift > 0;
            let mut betw = Bitboard::ZERO;
            let mut cp = sq1b;
            loop {
                if shift_left {
                    cp <<= total_shift as u32;
                } else {
                    cp >>= total_shift.unsigned_abs();
                }

                if (cp & sq2b).nonzero() {
                    break;
                }
                betw |= cp;
            }

            unsafe {
                BETWEEN_SQUARES[s1.inner() as usize][s2.inner() as usize] = betw;
            }
        });
    });
}
