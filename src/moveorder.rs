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

use crate::prelude::Bitboard;
use crate::prelude::PType::*;
use crate::prelude::{bishop_moves, rook_moves, Position};
use crate::prelude::{MType::*, Move};

// A higher rating is worse because that means we are sacrificing
fn can_be_captured_rating(pos: &Position, mv: Move) -> f64 {
    let from = mv.from();
    let to = mv.to();
    let p = pos.piece_on(from);
    let us = pos.to_move();
    let ty = p.kind();
    debug_assert!(p.is_ok());

    let possible_ep = if pos.state().ep().is_ok() {
        Bitboard::from(pos.state().ep())
    } else {
        Bitboard::ZERO
    };

    let att = pos.attacks_to_occ(to, pos.all() ^ from ^ to ^ possible_ep);
    let pcs = att & pos.color(!us);
    let can_recapture = if (att & pos.color(us)).nonzero() {
        1.0
    } else {
        0.0
    };

    if pcs.zero() {
        return 0.0;
    }

    for possible_capture_ty in [Pawn, Knight, Bishop, Rook, Queen] {
        if (pcs & pos.piece(possible_capture_ty)).nonzero() {
            return ty.valuef() - (possible_capture_ty.valuef() * can_recapture);
        }
    }

    0.0
}

// A higher rating is better, 0 is no capture
fn is_capturing_rating(pos: &Position, mv: Move) -> f64 {
    let from = mv.from();
    let to = mv.to();
    let m = pos.piece_on(from);
    let cap = pos.piece_on(to);

    if pos.state().ep().is_ok() {
        return 0.5;
    }

    if !cap.is_ok() {
        return 0.0;
    }

    cap.kind().valuef() - m.kind().valuef()
}

fn rate_move(pos: &Position, mv: Move) -> f64 {
    let us = pos.to_move();
    let sac_rating = can_be_captured_rating(pos, mv);
    let win_material_rating = is_capturing_rating(pos, mv);

    // Losing your castling rights if not actually castling is bad
    let lose_castle_rights =
        if pos.state().cur_castle().castle_for(us) != (false, false) && mv.from() == pos.king(us) {
            if mv.kind() == Castle {
                1.0
            } else {
                0.7
            }
        } else {
            1.0
        };

    (win_material_rating * lose_castle_rights) - sac_rating
}

pub fn reorder_moves(pos: &Position, list: &mut Vec<Move>) {
    let mut new_vec: Vec<(f64, Move)> = Vec::with_capacity(list.len());

    for mv in list.iter() {
        new_vec.push((rate_move(pos, *mv), *mv));
    }

    new_vec.sort_by(|(s1, _), (s2, _)| s1.partial_cmp(s2).unwrap());
    let new_vec = new_vec.iter().map(|&(_, m)| m).collect::<Vec<_>>();

    *list = new_vec;
}
