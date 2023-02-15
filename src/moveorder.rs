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

use crate::prelude::Color;
use crate::prelude::Square;
use crate::prelude::Position;
use crate::prelude::{MType::*, Move, MoveList};
use crate::prelude::PType::{self, *};
use crate::prelude::Piece;

static PAWN_WT: [f64; 32] = [
    0.00, 0.00, 0.00, 0.00, 1.00, 1.00, 1.00, 1.00, 0.80, 0.81, 0.83, 0.92, 0.60, 0.75, 0.77, 0.79,
    0.55, 0.70, 0.75, 0.75, 0.40, 0.40, 0.40, 0.40, 0.25, 0.25, 0.25, 0.25, 0.00, 0.00, 0.00, 0.00,
];

static KNIGHT_WT: [f64; 32] = [
    0.10, 0.15, 0.10, 0.08, 0.10, 0.30, 0.38, 0.38, 0.10, 0.38, 0.45, 0.50, 0.14, 0.50, 0.65, 0.75,
    0.10, 0.50, 0.65, 0.75, 0.14, 0.38, 0.45, 0.50, 0.10, 0.30, 0.38, 0.38, 0.10, 0.15, 0.10, 0.08,
];

static BISHOP_WT: [f64; 32] = [
    0.38, 0.20, 0.25, 0.21, 0.35, 0.40, 0.27, 0.30, 0.40, 0.32, 0.31, 0.25, 0.38, 0.45, 0.38, 0.32,
    0.38, 0.45, 0.38, 0.32, 0.40, 0.32, 0.31, 0.25, 0.35, 0.40, 0.27, 0.30, 0.38, 0.20, 0.25, 0.21,
];

static ROOK_WT: [f64; 32] = [
    0.40, 0.40, 0.40, 0.40, 0.90, 0.90, 0.90, 0.90, 0.20, 0.30, 0.30, 0.45, 0.25, 0.28, 0.28, 0.35,
    0.25, 0.27, 0.28, 0.35, 0.30, 0.22, 0.25, 0.45, 0.22, 0.29, 0.25, 0.45, 0.30, 0.35, 0.40, 0.60,
];

static KING_WT: [f64; 32] = [
    0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01,
    0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.15, 0.15, 0.15, 0.15, 0.72, 0.80, 0.80, 0.15,
];

fn mvv_lva(pos: &Position, m: Move) -> f64 {
    let cap = if m.kind() == EnPassant {
        Pawn
    } else {
        pos.piece_on(m.to()).kind()
    };
    let mover = pos.piece_on(m.from()).kind();
    let mover = if m.kind() == Promotion {
        m.promo()
    } else {
        mover
    };
    cap.valuef() * 100.0 - mover.valuef() * 90.0
}

fn lva_to(pos: &Position, square: Square) -> (Piece, Square) {
    let possible_attackers = pos.attacks_to(square) & pos.color(pos.to_move());
    for pt in [Pawn, Knight, Bishop, Rook, Queen, King] {
        let k = possible_attackers & pos.spec(pt, pos.to_move());
        if k.nonzero() {
            return (Piece::new(pt, pos.to_move()), k.get_square());
        }
    }

    (Piece::NULL, Square::NULL)
}
fn static_exchange_eval(pos: &mut Position, square: Square) -> f64 {
    todo!("SEE on ChessProgramming Wiki");
    let mut val = 0.0;

    let (p, s) = lva_to(pos, square);

    let mut occ = pos.all();
    if p.is_ok() {

    }

    val
}

fn piece_map_wt(square: Square, ty: PType, color: Color) -> f64 {
    let index = square.weight_map_idx(color);
    match ty {
        Pawn => PAWN_WT[index],
        Knight => KNIGHT_WT[index],
        Bishop => BISHOP_WT[index],
        Rook => ROOK_WT[index],
        Queen => (ROOK_WT[index] + BISHOP_WT[index]) / 2.0,
        King => KING_WT[index],
    }
}

pub fn order_moves(pos: &Position, move_list: &mut MoveList) {
    let mut mapped_moves: Vec<((f64, Move), i32)> = Vec::with_capacity(256);
    let us = pos.to_move();
    let them = !us;

    for i in 0..move_list.len() {
        let m = move_list.get(i);
        debug_assert!(m.is_ok());
        let to = m.to();
        let from = m.from();
        let moved = pos.piece_on(from);
        let cap = pos.piece_on(to);

        debug_assert!(!cap.is_ok() || cap.color() == them);
        debug_assert!(moved.is_ok());
        debug_assert_eq!(moved.color(), us);
        // It's too costly in regular dev builds.
        #[cfg(feature = "diagnostics")]
        debug_assert!(pos.is_legal(m));

        // TODO Killer move stuff
        let (score, prio_grp) = 'sprog: {
            if m.kind() == Promotion || cap.is_ok() || m.kind() == EnPassant {
                let cap_s = if cap.is_ok() || m.kind() == EnPassant {
                    mvv_lva(pos, m)
                } else {
                    0.0
                };

                let prom_s = if m.kind() == Promotion {
                    m.promo().valuef() * 100.0
                } else {
                    0.0
                };

                let s = prom_s + cap_s;

                let prio = if cap_s < 0.0 {
                    4
                } else if cap_s == 0.0 {
                    2
                } else {
                    1
                };

                break 'sprog (s, prio);
            } else {
                let dest_wt = piece_map_wt(to, moved.kind(), moved.color());
                let from_wt = piece_map_wt(from, moved.kind(), moved.color());
                let check_wt = f64::from(pos.gives_check(m) as i32) * 400.0;
                let s = dest_wt - from_wt + check_wt;
                break 'sprog (s, 3);
            }
        };

        mapped_moves.push(((score, m), prio_grp));
    }

    mapped_moves.sort_by(|((s1, _), p1), ((s2, _), p2)| {
        if p1 != p2 {
            p1.cmp(p2)
        } else {
            s1.partial_cmp(s2).unwrap()
        }
    });

    for (i, &((_, m), _)) in mapped_moves.iter().enumerate() {
        move_list.set(i, m);
    }
}
