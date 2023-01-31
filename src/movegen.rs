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

use crate::chessmove::{MType, Move};
use crate::position::{Position, State, Castle};
use crate::bitboard::Bitboard;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GenType {
    Captures,
    Evasions,
    NonEvasions,
    Quiet,
    QuietChecks,
}

pub fn generate_all(pos: &Position, list: &mut Vec<Move>, gt: GenType) {
}

pub fn generate_legal(pos: &Position, list: &mut Vec<Move>) {
    let us = pos.to_move();
    let gt = if pos.state().checkers().nonzero() {
        GenType::NonEvasions
    } else {
        GenType::Evasions
    };

    generate_all(pos, list, gt);
}
