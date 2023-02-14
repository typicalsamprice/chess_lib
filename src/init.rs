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

use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::filerank::File;
use crate::magic::{bishop_moves, rook_moves};
use crate::square::Square;

static mut KNIGHT_ATTACKS: [Bitboard; 64] = Bitboard::arr::<64>();
static mut KING_ATTACKS: [Bitboard; 64] = Bitboard::arr::<64>();
static mut PAWN_ATTACKS: [[Bitboard; 2]; 64] = Bitboard::arr_2d::<2, 64>();
static mut BETWEEN_SQUARES: [[Bitboard; 64]; 64] = Bitboard::arr_2d::<64, 64>();
static mut LINE_BB: [[Bitboard; 64]; 64] = Bitboard::arr_2d::<64, 64>();

pub fn init() {
    if unsafe { PAWN_ATTACKS[0][0] } != Bitboard::ZERO {
        panic!("Do not call init::comp_init() many times");
    }
    init_pawn_attacks();
    init_knight_attacks();
    init_king_attacks();
    // Required first. Because between() uses Square::in_line in the setup
    init_between_and_board_lines();
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
pub fn between<const INCLUDE_ENDPOINT: bool>(s1: Square, s2: Square) -> Bitboard {
    if INCLUDE_ENDPOINT {
        unsafe { BETWEEN_SQUARES[s1.inner() as usize][s2.inner() as usize] | s2 }
    } else {
        unsafe { BETWEEN_SQUARES[s1.inner() as usize][s2.inner() as usize] }
    }
}
pub fn line(s1: Square, s2: Square) -> Bitboard {
    unsafe { LINE_BB[s1.inner() as usize][s2.inner() as usize] }
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
fn init_between_and_board_lines() {
    for i in 0..64 {
        for j in 0..64 {
            let si = unsafe { Square::new(i as u8) };
            let sj = unsafe { Square::new(j as u8) };

            if si == sj {
                continue;
            }

            let rook_si = rook_moves(si, Bitboard::ZERO);
            let bish_si = bishop_moves(si, Bitboard::ZERO);
            let b_rook_si = rook_moves(si, Bitboard::from(sj));
            let b_bish_si = bishop_moves(si, Bitboard::from(sj));

            let line = if (rook_si & sj).nonzero() {
                rook_si & rook_moves(sj, Bitboard::ZERO)
            } else if (bish_si & sj).nonzero() {
                bish_si & bishop_moves(sj, Bitboard::ZERO)
            } else {
                Bitboard::ZERO
            };

            let betw = if (b_rook_si & sj).nonzero() {
                b_rook_si & rook_moves(sj, Bitboard::from(si))
            } else if (b_bish_si & sj).nonzero() {
                b_bish_si & bishop_moves(sj, Bitboard::from(si))
            } else {
                Bitboard::ZERO
            };
            let others = Bitboard::from(si) | sj;
            let total = line | others;
            unsafe {
                LINE_BB[i][j] = total;
            }
            unsafe {
                BETWEEN_SQUARES[i][j] = betw;
            }
        }
    }
}
