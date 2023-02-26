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

use crate::prelude::PType::*;
use crate::prelude::Position;
use crate::prelude::pawn_attack;
use crate::prelude::{MType::*, MoveList};
use crate::prelude::ExtMove;

const CAP_SCORE: i32 = 10;
const CONTROL_BY_OPP_PAWN_SCORE: i32 = 350;
/* const TT_MOVE_SCORE: i32 = 10_000; */

pub fn order_moves(pos: &Position, move_list: &mut MoveList/*, tt: TransposeTable*/) {
    /* let hashm = tt.get_stored(); */
    for i in 0..move_list.len() {
        let mut score = 0;
        let ext = move_list.get_mut(i);
        let m = ext.unwrap();

        debug_assert!(m.is_ok());

        let from = m.from();
        let to = m.to();
        let k = m.kind();
        let prom = m.promo();

        let us = pos.to_move();
        let them = !us;

        let p = pos.piece_on(from);
        let cap = pos.piece_on(to);

        if cap.is_ok() {
            score = CAP_SCORE * cap.kind().value() - p.kind().value();
        }

        if p.kind() == Pawn {
            if k == Promotion {
                score += prom.value();
            }
        } else if (pawn_attack(to, us) & pos.spec(Pawn, them)).nonzero() {
            score -= CONTROL_BY_OPP_PAWN_SCORE;
        }

        // if m == hash {
        //     score += TT_MOVE_SCORE;
        // }

        *ext = ExtMove::new(m, score);
    }

    move_list.sort();
}
