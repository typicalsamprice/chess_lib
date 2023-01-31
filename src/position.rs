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

use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

use crate::prelude::*;
use Color::*;

#[derive(Debug)]
pub struct Position {
    board: [Piece; 64],
    pieces: [Bitboard; 6],
    colors: [Bitboard; 2],

    ply: i32,
    to_move: Color,
    state: State,
}

#[derive(Debug, Default)]
pub struct State {
    check_squares: [Bitboard; 6],
    castle: Castle,
    ep: Square, // Just have Square(64) for not-available
    rule50: i32,

    checkers: Bitboard,
    blockers: [Bitboard; 2],
    pinners: [Bitboard; 2],

    prev: Option<Rc<State>>
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Castle(u8);

impl Position {
    #[inline]
    pub const fn color(&self, color: Color) -> Bitboard {
        self.colors[color as usize]
    }
    #[inline]
    pub const fn piece(&self, ty: PType) -> Bitboard {
        self.pieces[ty as usize]
    }
    #[inline]
    pub const fn spec(&self, ty: PType, color: Color) -> Bitboard {
        self.color(color).const_and(self.piece(ty))
    }

    #[inline]
    pub const fn piece_2t(&self, ty1: PType, ty2: PType) -> Bitboard {
        self.piece(ty1).const_or(self.piece(ty2))
    }
    #[inline]
    pub const fn spec_2t(&self, ty1: PType, ty2: PType, color: Color) -> Bitboard {
        self.piece_2t(ty1, ty2).const_and(self.color(color))
    }
    #[inline]
    pub const fn all(&self) -> Bitboard {
        self.color(White).const_or(self.color(Black))
    }

    #[inline]
    pub const fn king(&self, color: Color) -> Square {
        self.spec(PType::King, color).get_square()
    }

    #[inline]
    pub const fn ply(&self) -> i32 {
        self.ply
    }
    #[inline]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[inline]
    pub const fn to_move(&self) -> Color {
        self.to_move
    }

    pub fn gives_check(&self, m: Move) -> bool {
        let p = self.piece_on(m.from());
        let dest = m.to();
        let cap = self.piece_on(dest);
        debug_assert!(p.is_ok());
        debug_assert_eq!(p.color(), self.to_move());
        debug_assert!(!cap.is_ok() || cap.color() != self.to_move());

        // TODO: Discovered check
        (self.state().check_squares(p.kind()) & dest).nonzero()
    }

    pub fn attacks_to(&self, square: Square) -> Bitboard {
        let pawns = (pawn_attack(square, White) & self.spec(PType::Pawn, Black))
            | (pawn_attack(square, Black) & self.spec(PType::Pawn, White));
        let knights = knight_attack(square) & self.piece(PType::Knight);
        let kings = king_attack(square) & self.piece(PType::King);
        let bish = bishop_moves(square, self.all()) & self.piece_2t(PType::Bishop, PType::Queen);
        let rook = rook_moves(square, self.all()) & self.piece_2t(PType::Rook, PType::Queen);

        pawns | knights | kings | bish | rook
    }

    #[inline]
    fn add_piece(&mut self, square: Square, piece: Piece) {
        debug_assert!(self.is_empty_square(square));
        self.board[square.inner() as usize] = piece;
        self.colors[piece.color() as usize] |= square;
        self.pieces[piece.kind() as usize] |= square;
    }
    #[inline]
    fn clear_square(&mut self, square: Square) -> Piece {
        let p = self.board[square.inner() as usize];


        if p != Piece::NULL {
            self.board[square.inner() as usize] = Piece::NULL;
            self.colors[p.color() as usize] ^= square;
            self.pieces[p.kind() as usize] ^= square;
        }

        p
    }

    #[inline]
    pub fn piece_on(&self, square: Square) -> Piece {
        self.board[square.inner() as usize]
    }
    #[inline]
    pub fn is_empty_square(&self, square: Square) -> bool {
        self.piece_on(square).inner() == Piece::NULL.inner()
    }

    fn compute_state(&mut self) {
        let us = self.to_move();
        let king = self.king(us);
        self.state.checkers = self.attacks_to(king) & self.color(!us);
        self.state.pinners[0] = Bitboard::ZERO;
        self.state.pinners[1] = Bitboard::ZERO;
        self.state.blockers[0] = Bitboard::ZERO;
        self.state.blockers[1] = Bitboard::ZERO;

        self.state.check_squares[0] = pawn_attack(king, us);
        self.state.check_squares[1] = knight_attack(king);
        self.state.check_squares[2] = bishop_moves(king, Bitboard::ZERO);
        self.state.check_squares[3] = rook_moves(king, Bitboard::ZERO);
        self.state.check_squares[4] = self.state.check_squares[2] | self.state.check_squares[3];
        self.state.check_squares[5] = if (self.state.blockers(us) & self.king(!us)).nonzero() {
            king_attack(self.king(!us))
        } else {
            Bitboard::ZERO
        };

        for col in [White, Black] {
            let k = self.king(col);
            let z = Bitboard::ZERO;
            let rooks = rook_moves(k, z) & self.spec_2t(PType::Rook, PType::Queen, !col);
            let bish = bishop_moves(k, z) & self.spec_2t(PType::Bishop, PType::Queen, !col);
            let sliders = rooks | bish;
            sliders.map_by_square(|sq| {
                let b = between(k, sq) & self.all();
                if b.popcnt() != 1 { return; }
                let p = self.piece_on(b.get_square());
                debug_assert_ne!(p, Piece::NULL);
                if p.color() == col {
                    self.state.blockers[col as usize] |= b;
                }
            });
        }
    }
}

impl State {
    #[inline]
    pub const fn check_squares(&self, ty: PType) -> Bitboard {
        self.check_squares[ty as usize]
    }
    #[inline]
    pub const fn ep(&self) -> Square {
        self.ep
    }
    #[inline]
    pub const fn rule50(&self) -> i32 {
        self.rule50
    }

    #[inline]
    pub const fn cur_castle(&self) -> Castle {
        self.castle
    }

    #[inline]
    pub const fn checkers(&self) -> Bitboard {
        self.checkers
    }
    #[inline]
    pub const fn blockers(&self, color: Color) -> Bitboard {
        self.blockers[color as usize]
    }
    #[inline]
    pub const fn pinners(&self, color: Color) -> Bitboard {
        self.pinners[color as usize]
    }

    #[inline]
    pub fn consume(self) -> Rc<Self> {
        self.prev.unwrap()
    }
}

impl Castle {
    #[inline]
    const fn inner(self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn castle_for(self, color: Color) -> (bool, bool) {
        let king = 1 << (color as u8 * 2);
        let queen = 1 << (color as u8 * 2 + 1);
        let c = self.inner();
        (c & king > 0, c & queen > 0)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            board: [Piece::NULL; 64],
            pieces: [Bitboard::ZERO; 6],
            colors: [Bitboard::ZERO; 2],
            ply: 0,
            to_move: White,
            state: State::default()
        }
    }
}

impl FromStr for Position {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut schars = s.chars();
        let mut p = Position::default();

        let mut s = 56;
        for c in schars.by_ref() {
            if c == ' ' { break; }
            if c.is_ascii_digit() {
                debug_assert!(c != '0' && c != '9');
                s += c as u8 - b'0';
            } else if c == '/' {
                s -= 16;
            } else {
                if let Ok(pc) = Piece::try_from(c) {
                    let sq = unsafe { Square::new(s) };
                    debug_assert!(sq.is_ok());
                    p.add_piece(sq, pc);
                } else {
                    return Err("Invalid piece char found");
                }
                s += 1;
            }
        }

        let Some(c) = schars.next() else {
            return Err("No color field given");
        };

        if c == 'w' {
            p.to_move = White;
        } else if c == 'b' {
            p.to_move = Black;
        } else {
            return Err("Invalid color given");
        }

        if schars.next() != Some(' ') {
            return Err("No field 3");
        }

        for c in schars.by_ref() {
            if c == ' ' { break; }
            else if c == '-' {
                debug_assert_eq!(p.state.castle.inner(), 0);
                if schars.next() != Some(' ') {
                    return Err("No field 4 given");
                }
                break;
            }

            match c {
                'K' => {
                    debug_assert_eq!(p.state.castle.inner() & 1, 0);
                    p.state.castle.0 |= 1;
                },
                'Q' => {
                    debug_assert_eq!(p.state.castle.inner() & 2, 0);
                    p.state.castle.0 |= 2;
                },
                'k' => {
                    debug_assert_eq!(p.state.castle.inner() & 4, 0);
                    p.state.castle.0 |= 4;
                },
                'q' => {
                    debug_assert_eq!(p.state.castle.inner() & 8, 0);
                    p.state.castle.0 |= 8;
                },
                _ => return Err("Unknown castling character"),
            }
        }

        if let Some(c) = schars.next() {
            if c == '-' {
                p.state.ep = unsafe { Square::new(64) };
            } else {
                let Ok(f) = File::try_from(c) else {
                    return Err("Invalid file given");
                };
                let Some(cn) = schars.next() else {
                    return Err("No rank given for EP");
                };
                let Ok(r) = Rank::try_from(cn) else {
                    return Err("Invalid rank given");
                };
                p.state.ep = Square::create(f, r);
            }
        } else {
            return Err("No EP specifier");
        }

        p.compute_state();
        Ok(p)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::with_capacity(200);
        let sep = " +---+---+---+---+---+---+---+---+\n";

        for i in 0..8 {
            s.push_str(sep);
            for j in 0..8 {
                s.push_str(" | ");
                let k = (8 * (7 - i)) + j;
                let sq = unsafe { Square::new(k) };
                let p = self.piece_on(sq);

                if p.is_ok() {
                    s.push_str(&p.to_string());
                } else {
                    s.push(' ');
                }
            }
            s.push_str(" | ");
            s.push((b'8' - i) as char);
            s.push('\n');
        }
        s.push_str(sep);
        s.push_str("   a   b   c   d   e   f   g   h\n");

        write!(f, "{s}")
    }
}
