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

use crate::prelude::Move;
use crate::prelude::{generate_for, generate_legal, MType, MoveList};
use crate::prelude::{Color, Position};

fn material_balance(pos: &Position) -> f64 {
    let white_material = pos.material(Color::White);
    let black_material = pos.material(Color::Black);

    white_material - black_material
}

pub fn static_evaluate(pos: &Position) -> f64 {
    material_balance(pos)
}

pub fn minimax<const ROOT: bool>(pos: &mut Position, best_move: &mut Move, depth: usize) -> f64 {
    if depth == 0 {
        return pos.to_move().persp(static_evaluate(pos));
    }
    let mut move_list = MoveList::new();
    let mut best_rat = f64::NEG_INFINITY;
    generate_legal::<false>(pos, &mut move_list);
    for i in 0..move_list.len() {
        let m = move_list.get(i);
        pos.do_move(m);
        let e = -minimax::<false>(pos, best_move, depth - 1);
        if e > best_rat {
            best_rat = e;
            if ROOT {
                *best_move = m;
            }
        }
        pos.undo_move(m);
    }

    best_rat
}

pub fn alpha_beta<const ROOT: bool>(
    pos: &mut Position,
    best_move: &mut Move,
    alpha: f64,
    beta: f64,
    depth: usize,
) -> f64 {
    if depth == 0 {
        // TODO Quiescent search
        return pos.to_move().persp(static_evaluate(pos));
    }
    let mut move_list = MoveList::new();
    generate_legal::<false>(pos, &mut move_list);
    let mut alpha = alpha;
    for i in 0..move_list.len() {
        let m = move_list.get(i);
        pos.do_move(m);
        let e = -alpha_beta::<false>(pos, best_move, -alpha, -beta, depth - 1);
        pos.undo_move(m);
        if e >= beta {
            return beta;
        }
        if e > alpha {
            alpha = e;
            *best_move = m;
        }
    }

    alpha
}
