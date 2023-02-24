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

fn material_balance(pos: &Position) -> f64 {
    let white_material = pos.material(Color::White);
    let black_material = pos.material(Color::Black);

    white_material - black_material
}

pub fn static_evaluate(pos: &Position) -> f64 {
    let mut move_list = MoveList::new();
    generate_legal::<false>(pos, &mut move_list);
    if move_list.len() > 0 {
        material_balance(pos)
    } else if pos.in_check() {
        f64::NEG_INFINITY
    } else {
        0.0
    }
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

pub fn alpha_beta<const PARALLEL: bool>(pos: &mut Position, best_move: &mut Move, depth: usize) -> f64 {
    if PARALLEL {
        diagnostics::reset_alphabeta_leaf_nodes();
        diagnostics::reset_beta_cutoffs();
        let mut _useless = Move::NULL;
        let mut move_list = MoveList::new();
        generate_legal::<false>(pos, &mut move_list);
        let mut alpha = pos.to_move().persp(f64::NEG_INFINITY);
        let beta = pos.to_move().persp(f64::INFINITY);
        let mut handles = vec![];

        if move_list.len() == 0 {
            return pos.to_move().persp(static_evaluate(pos));
        }
        else if depth == 0 {
            panic!("alpha_beta called with parallel and depth 0");
        }

        for i in 0..move_list.len() {
            let mut cp = pos.clone();
            let m = move_list.get(i);
            cp.do_move(m);
            debug!("Starting thread #{}", i);

            let h = thread::spawn(move || {
                let e = alpha_beta_internal::<false>(&mut cp, &mut _useless,
                                                     depth - 1, -beta, -alpha);
                (e, m)
            });
            handles.push(h);
        }
        for h in handles {
            let (e, m) = h.join().unwrap();
            if e >= beta {
                return beta;
            }

            if e > alpha {
                alpha = e;
                *best_move = m;
            }
        }

        alpha
    } else {
        let tm = pos.to_move();
        alpha_beta_internal::<true>(
            pos,
            best_move,
            depth,
            tm.persp(f64::NEG_INFINITY),
            tm.persp(f64::INFINITY),
        )
    }
}

fn alpha_beta_internal<const ROOT: bool>(
    pos: &mut Position,
    best_move: &mut Move,
    depth: usize,
    alpha: f64,
    beta: f64,
) -> f64 {
    if ROOT {
        diagnostics::reset_beta_cutoffs();
        diagnostics::reset_alphabeta_leaf_nodes();
    }

    let mut move_list = MoveList::new();
    generate_legal::<false>(pos, &mut move_list);

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
        let m = move_list.get(i);
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

pub(crate) fn quiescence(pos: &mut Position, alpha: f64, beta: f64) -> f64 {
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
    let gt = if pos.state().checkers().nonzero() {
        GenType::Evasions
    } else {
        GenType::Captures
    };
    generate_for(pos, &mut move_list, pos.to_move(), gt);

    for i in 0..move_list.len() {
        let m = move_list.get(i);
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
