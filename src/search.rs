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

use crate::MAX_PLY;
use crate::diagnostics;
use crate::prelude::*;
use crate::evaluate;
use crate::moveorder::order_moves;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    len: usize,
    moves: [Move; MAX_PLY]
}

impl Line {
    pub const fn new() -> Self {
        Self { len: 0, moves: [Move::NULL; MAX_PLY] }
    }

    pub const fn len(&self) -> usize {
        self.len
    }
    pub fn set(&mut self, idx: usize, m: Move) {
        if self.len <= idx {
            self.len = idx + 1;
        }
        self.moves[idx] = m;
    }

    #[inline]
    pub fn as_slice<'a>(&'a self) -> &'a [Move] {
        &self.moves[0..self.len]
    }
}

pub fn ab_with_pv(pos: &mut Position, depth: usize) -> (Line, i32) {
    let c = pos.to_move();
    let mut l = Line::new();
    let eval = ab_compile_lines(pos, depth, c.persp(-i32::MAX), c.persp(i32::MAX), &mut l);
    (l, eval)
}

fn ab_compile_lines(pos: &mut Position, depth: usize, alpha: i32, beta: i32, pv: &mut Line) -> i32 {
    let mut move_list = MoveList::new();
    let mut line = Line::new();
    let mut alpha = alpha;

    if depth == 0 {
        return evaluate::static_evaluate(pos);
    }

    generate_legal::<false>(pos, &mut move_list);
    for i in 0..move_list.len() {
        let m = move_list.get(i);
        pos.do_move(m);
        let e = -ab_compile_lines(pos, depth - 1, -beta, -alpha, &mut line);
        pos.undo_move(m);

        if e >= beta {
            diagnostics::add_beta_cutoffs();
            return beta;
        }
        if e > alpha {
            alpha = e;
            pv.len = line.len() + 1;
            pv.set(0, m);
            let src = &line.moves[0] as *const _;
            let dst = &mut pv.moves[1] as *mut _;
            let count = line.len();
            unsafe {
                std::ptr::copy(src, dst, count);
            }
        }
    }

    alpha
}
