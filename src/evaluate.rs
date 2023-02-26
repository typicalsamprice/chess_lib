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

use std::thread;

use crate::diagnostics;
use crate::moveorder::order_moves;
use crate::prelude::{generate_for, generate_legal, MoveList};
use crate::prelude::{Color, Position};
use crate::prelude::{GenType, Move};

use crate::debug;

// These contants make it easier to read.
pub const MAX_RAT: i32 = i32::MAX;
pub const MIN_RAT: i32 = -i32::MAX;

fn material_balance(pos: &Position) -> i32 {
    let white_material = pos.material(Color::White);
    let black_material = pos.material(Color::Black);

    white_material - black_material
}

pub fn static_evaluate(pos: &Position) -> i32 {
    let mut move_list = MoveList::new();
    generate_legal(pos, &mut move_list);
    if move_list.len() > 0 {
        material_balance(pos)
    } else if pos.in_check() {
        MIN_RAT
    } else {
        0
    }
}

pub fn minimax<const ROOT: bool>(pos: &mut Position, best_move: &mut Move, depth: usize) -> i32 {
    if depth == 0 {
        return pos.to_move().persp(static_evaluate(pos));
    }
    let mut move_list = MoveList::new();
    let mut best_rat = i32::MIN;
    generate_legal(pos, &mut move_list);
    for i in 0..move_list.len() {
        let ext = move_list.get(i);
        let m = ext.unwrap();
        if !pos.is_legal(m) {
            continue;
        }
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

pub fn alpha_beta(pos: &mut Position, best_move: &mut Move, depth: usize) -> i32 {
    let tm = pos.to_move();
    alpha_beta_internal::<true>(pos, best_move, depth,
                                tm.persp(MIN_RAT),
                                tm.persp(MAX_RAT))
}

fn alpha_beta_internal<const ROOT: bool>(
    pos: &mut Position, best_move: &mut Move, depth: usize,
    alpha: i32, beta: i32
) -> i32 {
    if ROOT {
        diagnostics::reset_beta_cutoffs();
        diagnostics::reset_alphabeta_leaf_nodes();
    }

    let mut move_list = MoveList::new();
    generate_legal(pos, &mut move_list);

    if move_list.len() == 0 || depth == 0 {
        diagnostics::add_alphabeta_leaf_nodes();
    }

    if move_list.len() == 0 {
        return pos.to_move().persp(static_evaluate(pos));
    } else if depth == 0 {
        return quiescence(pos, alpha, beta);
    }

    order_moves(pos, &mut move_list);
    let mut alpha = alpha;

    for i in 0..move_list.len() {
        let ext = move_list.get(i);
        let m = ext.unwrap();
        pos.do_move(m);
        let se = -alpha_beta_internal::<false>(pos, best_move, depth - 1, -beta, -alpha);
        pos.undo_move(m);

        if se >= beta {
            diagnostics::add_beta_cutoffs();
            return beta;
        }

        if se > alpha {
            alpha = se;
            if ROOT {
                debug!("ROOT: {} {} {}", se, alpha, m);
                *best_move = m;
            }
        }
    }

    alpha
}

pub(crate) fn quiescence(pos: &mut Position, alpha: i32, beta: i32) -> i32 {
    let stand_pat = pos.to_move().persp(static_evaluate(pos));
    let mut alpha = alpha;

    if stand_pat >= beta {
        diagnostics::add_beta_cutoffs();
        return beta;
    }

    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut move_list = MoveList::new();
    let gt = if pos.in_check() {
        GenType::Evasions
    } else {
        GenType::Captures
    };
    generate_for(pos, &mut move_list, pos.to_move(), gt);

    for i in 0..move_list.len() {
        let ext = move_list.get(i);
        let m = ext.unwrap();
        if !pos.is_legal(m) {
            continue;
        }
        pos.do_move(m);
        let e = -quiescence(pos, -beta, -alpha);
        pos.undo_move(m);

        if e >= beta {
            diagnostics::add_beta_cutoffs();
            return beta;
        }

        if e > alpha {
            alpha = e;
        }
    }

    alpha
}
