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

use crate::prelude::*;
use crate::evaluate;
use crate::prelude::individual_squares::A1;

#[derive(Debug)]
pub struct Engine<'elt> {
    pos: Position,
    /*uci_opts: &'elt UciOptions, TODO Make UciOptions */
}

impl<'elt> Engine<'elt> {
    pub fn new(/*uci_opts: &'elt UciOptions*/) -> Self {
        Self {
            pos: Position::default(),
            /* uci_opts, */
        }
    }

    pub fn initialize(&mut self) {
        // Parse UciOptions to set up correctly
        todo!();
    }

    pub fn search(&mut self, depth: usize) -> Move {
        let mut m = Move::new(A1, A1);
        evaluate::alpha_beta(&mut self.pos, &mut m, depth);
        return m;
    }
}
