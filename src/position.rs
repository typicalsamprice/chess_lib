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
use crate::prelude::individual_squares::*;
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

#[derive(Debug, Default, Clone)]
pub struct State {
    check_squares: [Bitboard; 6],
    castle: Castle,
    ep: Square, // Just have Square(64) for not-available
    rule50: i32,

    checkers: Bitboard,
    blockers: [Bitboard; 2],
    pinners: [Bitboard; 2],

    captured: Piece,

    prev: Option<Rc<State>>,
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

        let is_moving_to_check = (self.state().check_squares(p.kind()) & dest).nonzero();
        let blocker = self.state().blockers(!self.to_move()) & m.from();
        let is_blocker = blocker.nonzero();

        if !is_blocker {
            return is_moving_to_check;
        }

        let is_discovery = if !is_blocker {
            false
        } else {
            let k = self.king(!self.to_move());
            let mut discoverer = Square::NULL;
            self.state()
                .pinners(self.to_move())
                .map_by_square(|pinner| {
                    if discoverer.is_ok() {
                        return;
                    }
                    if pinner.in_line2(blocker.get_square(), k) {
                        discoverer = pinner;
                        return;
                    }
                });

            discoverer.in_line2(blocker.get_square(), k)
        };

        is_discovery || is_moving_to_check
    }

    pub fn attacks_to(&self, square: Square) -> Bitboard {
        self.attacks_to_occ(square, self.all())
    }
    pub fn attacks_to_occ(&self, square: Square, occ: Bitboard) -> Bitboard {
        let pawns = (pawn_attack(square, White) & self.spec(PType::Pawn, Black))
            | (pawn_attack(square, Black) & self.spec(PType::Pawn, White));
        let knights = knight_attack(square) & self.piece(PType::Knight);
        let kings = king_attack(square) & self.piece(PType::King);
        let bish = bishop_moves(square, occ) & self.piece_2t(PType::Bishop, PType::Queen);
        let rook = rook_moves(square, occ) & self.piece_2t(PType::Rook, PType::Queen);

        pawns | knights | kings | bish | rook
    }

    fn slider_blockers(
        &self,
        sliders: Bitboard,
        square: Square,
    ) -> (Bitboard, Bitboard) {
        let mut blockers = Bitboard::ZERO;
        let mut pinners = Bitboard::ZERO;
        let snipers = ((rook_moves(square, Bitboard::ZERO) & self.piece_2t(PType::Rook, PType::Queen))
                | (bishop_moves(square, Bitboard::ZERO) & self.piece_2t(PType::Bishop, PType::Queen))) & sliders;

        let occ = self.all() ^ snipers;
        snipers.map_by_square(|sniper| {
            let b = between::<false>(square, sniper) & occ;
            if b.nonzero() && !b.more_than_one() {
                blockers |= b;
                if (b & self.color(self.piece_on(square).color())).nonzero() {
                    pinners |= sniper;
                }
            }
        });

        (blockers, pinners)
    }

    #[inline]
    fn add_piece(&mut self, square: Square, piece: Piece) {
        debug_assert!(self.is_empty_square(square));
        debug_assert!(piece.is_ok());
        self.board[square.inner() as usize] = piece;
        self.colors[piece.color() as usize] |= square;
        self.pieces[piece.kind() as usize] |= square;
    }
    #[inline]
    fn clear_square(&mut self, square: Square) -> Piece {
        let p = self.board[square.inner() as usize];

        if p.is_ok() {
            self.board[square.inner() as usize] = Piece::NULL;
            self.colors[p.color() as usize] ^= square;
            self.pieces[p.kind() as usize] ^= square;
        }

        p
    }

    pub fn is_pseudo_legal(&self, mv: Move) -> bool {
        todo!();
    }
    pub fn is_legal(&self, mv: Move) -> bool {
        let us = self.to_move();
        let k = self.king(us);
        let from = mv.from();
        let to = mv.to();
        let ty = mv.kind();
        let moved = self.piece_on(from);
        let cap = self.piece_on(to);

        debug_assert_eq!(moved.color(), us);
        debug_assert_ne!(moved, Piece::NULL);

        if ty == MType::EnPassant {
            let bw = (!us).pawn_push();
            debug_assert_eq!(self.state().ep(), to);
            debug_assert_eq!(moved, Piece::new(PType::Pawn, us));
            debug_assert_eq!(cap, Piece::NULL);
            let pawn_cap_sq = bw(Bitboard::from(to)).get_square();
            let pawn_cap = self.piece_on(pawn_cap_sq);
            debug_assert_eq!(pawn_cap, Piece::new(PType::Pawn, !us));

            let occ = self.all() ^ from ^ to ^ pawn_cap_sq;
            return (rook_moves(k, occ) & self.spec_2t(PType::Queen, PType::Rook, !us)).zero()
                && (bishop_moves(k, occ) & self.spec_2t(PType::Queen, PType::Bishop, !us)).zero();
        } else if ty == MType::Castle {
            let ib = between::<true>(k, to);
            let mut valid = true;
            ib.map_by_square(|s| {
                if !valid {
                    return;
                }
                if (self.attacks_to(s) & self.color(!us)).nonzero() {
                    valid = false;
                }
            });

            return valid;
        }

        if from == k {
            return (self.attacks_to_occ(to, self.all() ^ from) & self.color(!us)).zero();
        }

        (self.state().blockers(us) & from).zero() || from.in_line2(to, k)
    }

    pub fn do_move(&mut self, mv: Move) {
        let from = mv.from();
        let to = mv.to();
        let ty = mv.kind();
        let prom = mv.promo();

        let us = self.to_move();

        debug_assert!(ty != MType::Promotion || (prom != PType::Pawn && prom != PType::King));
        debug_assert!(mv.is_ok());

        let moved = self.clear_square(from);
        debug_assert_ne!(moved, Piece::NULL);
        debug_assert_eq!(moved.color(), us);

        let cap = self.clear_square(to);
        debug_assert!(!cap.is_ok() || cap.color() == !us);
        debug_assert!(ty == MType::EnPassant || to != self.state().ep() || !cap.is_ok());
        debug_assert!(ty != MType::Castle || !cap.is_ok());
        debug_assert!(ty != MType::Castle || from == E1.relative(us));

        let mut st = self.state.clone();
        st.captured = cap;
        st.rule50 += 1;
        self.ply += 1;
        st.ep = Square::NULL;

        if ty != MType::Promotion {
            self.add_piece(to, moved);
        } else {
            let prom = Piece::new(mv.promo(), us);
            debug_assert!(prom.kind() != PType::Pawn && prom.kind() != PType::King);
            self.add_piece(to, prom);
        }

        if ty == MType::EnPassant {
            let ep_cap_sq = Color::pawn_push(!us)(Bitboard::from(to)).get_square();
            let c = self.clear_square(ep_cap_sq);
            debug_assert_eq!(c, Piece::new(PType::Pawn, !us));
            st.captured = c;
        } else if ty == MType::Castle {
            let rook_file = if to.file() == File::C {
                File::A
            } else {
                File::H
            };
            let rook_dest_file = if to.file() == File::C {
                File::D
            } else {
                File::F
            };
            let rook_square = Square::create(rook_file, from.rank());
            let rk = self.clear_square(rook_square);
            debug_assert_eq!(rk, Piece::new(PType::Rook, us));
            self.add_piece(Square::create(rook_dest_file, from.rank()), rk);
            // Remove all rights for that color
        }

        if moved.kind() == PType::King {
            st.castle = Castle(st.castle.inner() & !(3 << (2 * us as usize)));
        }

        if moved.kind() == PType::Rook {
            match from.relative(us) {
                A1 if st.castle.castle_for(us).1 => {
                    st.castle.0 ^= 2 << (2 * us as usize);
                },
                H1 if st.castle.castle_for(us).0 => {
                    st.castle.0 ^= 1 << (2 * us as usize);
                },
                _ => ()
            }
        }

        // Detect killing Their rooks
        if cap.is_ok() && cap.kind() == PType::Rook {
            let bit: u8 = match to.relative(us) {
                H8 => 1 << (2 * (!us) as u8),
                A8 => 2 << (2 * (!us) as u8),
                _ => 0,
            };
            st.castle.0 &= !bit;
        }

        if moved.kind() == PType::Pawn && from.dist(to) == 2 {
            let possible_ep = Color::pawn_push(us)(Bitboard::from(from)).get_square();
            if (pawn_attack(possible_ep, us) & self.spec(PType::Pawn, !us)).nonzero() {
                st.ep = possible_ep;
            }
        }

        if moved.kind() == PType::Pawn || cap.is_ok() {
            st.rule50 = 0;
        }

        std::mem::swap(&mut st, &mut self.state);
        self.state.prev = Some(Rc::new(st));
        self.to_move = !self.to_move;
        self.compute_state();
    }
    pub fn undo_move(&mut self, mv: Move) {
        let from = mv.from();
        let to = mv.to();
        let ty = mv.kind();
        let promo = mv.promo();
        let cap = self.state.captured;

        let mut st = None;
        std::mem::swap(&mut self.state.prev, &mut st);
        self.state =
            Rc::try_unwrap(st.unwrap()).expect("Undo-move tried to reset to nonexistent state");
        self.to_move = !self.to_move;
        let us = self.to_move();

        let mut moved = self.clear_square(to);
        debug_assert_eq!(moved.color(), us);
        if ty == MType::Promotion {
            debug_assert_eq!(moved, Piece::new(promo, us));
            moved = Piece::new(PType::Pawn, us);
        }
        self.add_piece(from, moved);
        if cap.is_ok() {
            let s = if ty == MType::EnPassant {
                Color::pawn_push(!us)(Bitboard::from(to)).get_square()
            } else {
                to
            };
            self.add_piece(s, cap);
        }

        if ty == MType::Castle {
            let (rook_on_now, rook_replace_to) = match to.file() {
                File::G => (File::F, File::H),
                File::C => (File::D, File::A),
                _ => panic!("Undoing invalid castle"),
            };
            let br = Rank::One.relative(us);
            let rk = self.clear_square(Square::create(rook_on_now, br));
            debug_assert_eq!(rk, Piece::new(PType::Rook, us));
            debug_assert!(!self.piece_on(Square::create(rook_replace_to, br)).is_ok());
            self.add_piece(Square::create(rook_replace_to, br), rk);
        }

        self.ply -= 1;
    }

    #[allow(non_upper_case_globals)]
    pub fn perft<const Root: bool>(&mut self, depth: usize) -> usize {
        assert_ne!(depth, 0);
        let mut nodes = 0;
        let mut cnt;
        let is_leaf = depth == 2;

        let mut moves = vec![];
        // Say 'false' here to skip a clear
        generate_legal::<false>(self, &mut moves);

        for mv in moves.clone() {
            if Root && depth == 1 {
                cnt = 1;
                nodes += 1;
            } else {
                self.do_move(mv);

                cnt = if is_leaf {
                    generate_legal::<true>(self, &mut moves);
                    moves.len()
                } else {
                    self.perft::<false>(depth - 1)
                };
                nodes += cnt;

                self.undo_move(mv);
            }

            if Root {
                println!("{mv}: {cnt}");
            }
        }

        nodes
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
        debug_assert_eq!(self.attacks_to(self.king(!us)) & self.color(us), Bitboard::ZERO);
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
            let king = self.king(col);
            let (blockers, pinners) = self.slider_blockers(self.color(!col), king);
            self.state.blockers[col as usize] = blockers;
            self.state.pinners[!col as usize] = pinners;
        }
    }

    pub fn fen(&self) -> String {
        let mut fen = String::with_capacity(92);

        macro_rules! f {
            () => {
                fen.push(' ');
            };
        }

        for i in 0..8 {
            let mut empty = 0;
            for j in 0..8 {
                let s = unsafe { Square::new((7 - i) * 8 + j) };
                debug_assert!(s.is_ok());
                let p = self.piece_on(s);

                if p.is_ok() {
                    if empty > 0 {
                        fen.push((b'0' + empty) as char);
                        empty = 0;
                    }
                    fen.push(char::from(p));
                } else {
                    empty += 1;
                }
            }
            if empty > 0 {
                fen.push((b'0' + empty) as char);
            }
            if i != 7 {
                fen.push('/');
            }
        }

        f!();
        fen.push(match self.to_move {
            White => 'w',
            _ => 'b',
        });
        f!();

        if self.state.castle.0 == 0 {
            fen.push('-');
        } else {
            let (wk, wq) = self.state.cur_castle().castle_for(White);
            let (bk, bq) = self.state.cur_castle().castle_for(Black);

            if wk {
                fen.push('K');
            }
            if wq {
                fen.push('Q');
            }
            if bk {
                fen.push('k');
            }
            if bq {
                fen.push('q');
            }
        }
        f!();

        if self.state.ep.is_ok() {
            fen.push_str(&self.state.ep.to_string());
        } else {
            fen.push('-');
        }

        // FIXME Add the halfmove and plies to the FEN

        fen
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
}

impl Castle {
    #[inline]
    const fn inner(self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn castle_for(self, color: Color) -> (bool, bool) {
        let king = match color {
            White => 1,
            Black => 4,
        };
        let queen = match color {
            White => 2,
            Black => 8,
        };
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
            state: State::default(),
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
            if c == ' ' {
                break;
            }
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
            if c == ' ' {
                break;
            } else if c == '-' {
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
                }
                'Q' => {
                    debug_assert_eq!(p.state.castle.inner() & 2, 0);
                    p.state.castle.0 |= 2;
                }
                'k' => {
                    debug_assert_eq!(p.state.castle.inner() & 4, 0);
                    p.state.castle.0 |= 4;
                }
                'q' => {
                    debug_assert_eq!(p.state.castle.inner() & 8, 0);
                    p.state.castle.0 |= 8;
                }
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

#[cfg(test)]
mod tests {
    use crate::prelude::{pawn_attack, rook_moves};
    use crate::prelude::individual_squares::A2;
    use crate::prelude::Color::White;
    use crate::prelude::Bitboard;
    use super::Position as Pos;
    use std::str::FromStr;

    const STARTPOS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const KIWI_FEN: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    const P3_FEN: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -";

    fn setup() {
        crate::init::init();
        crate::magic::initalize_magics();
    }

    #[test]
    fn depth_1() {
        setup();
        let mut p = Pos::from_str(STARTPOS_FEN).unwrap();
        let u = p.perft::<true>(1);
        assert_eq!(u, 20);
    }
    #[test]
    fn depth_2() {
        let mut p = Pos::from_str(STARTPOS_FEN).unwrap();
        let u = p.perft::<true>(2);
        assert_eq!(u, 400);
    }
    #[test]
    fn depth_3() {
        let mut p = Pos::from_str(STARTPOS_FEN).unwrap();
        let u = p.perft::<true>(3);
        assert_eq!(u, 8902);
    }
    #[test]
    fn depth_4() {
        let mut p = Pos::from_str(STARTPOS_FEN).unwrap();
        let u = p.perft::<true>(4);
        assert_eq!(u, 197_281);
    }
    #[test]
    fn depth_5() {
        let mut p = Pos::from_str(STARTPOS_FEN).unwrap();
        assert_eq!(p.perft::<true>(5), 4_865_609);
    }
    #[test]
    #[ignore]
    fn depth_6() {
        let mut p = Pos::from_str(STARTPOS_FEN).unwrap();
        assert_eq!(p.perft::<true>(6), 119_060_324);
    }
    #[test]
    #[ignore]
    fn depth_7() {
        let mut p = Pos::from_str(STARTPOS_FEN).unwrap();
        assert_eq!(p.perft::<true>(7), 3_195_901_860);
    }


    #[test]
    fn kiwi_depth_1() {
        let mut p = Pos::from_str(KIWI_FEN).unwrap();
        assert_eq!(p.perft::<true>(1), 48);
    }
    #[test]
    fn kiwi_depth_2() {
        let mut p = Pos::from_str(KIWI_FEN).unwrap();
        assert_eq!(p.perft::<true>(2), 2039);
    }
    #[test]
    fn kiwi_depth_3() {
        let mut p = Pos::from_str(KIWI_FEN).unwrap();
        assert_eq!(p.perft::<true>(3), 97_862);
    }
    #[test]
    fn kiwi_depth_4() {
        let mut p = Pos::from_str(KIWI_FEN).unwrap();
        assert_eq!(p.perft::<true>(4), 4_085_603);
    }
    #[test]
    fn kiwi_depth_5() {
        let mut p = Pos::from_str(KIWI_FEN).unwrap();
        assert_eq!(p.perft::<true>(5), 193_690_690);
    }
    #[test]
    #[ignore]
    fn kiwi_depth_6() {
        let mut p = Pos::from_str(KIWI_FEN).unwrap();
        assert_eq!(p.perft::<true>(6), 8_031_647_685);
    }

    #[test]
    fn fen3_depth_1() {
        let mut p = Pos::from_str(P3_FEN).unwrap();
        assert_eq!(p.perft::<true>(1), 14);
    }
    #[test]
    fn fen3_depth_2() {
        let mut p = Pos::from_str(P3_FEN).unwrap();
        assert_eq!(p.perft::<true>(2), 191);
    }
    #[test]
    fn fen3_depth_3() {
        let mut p = Pos::from_str(P3_FEN).unwrap();
        assert_eq!(p.perft::<true>(3), 2812);
    }
    #[test]
    fn fen3_depth_4() {
        let mut p = Pos::from_str(P3_FEN).unwrap();
        assert_eq!(p.perft::<true>(4), 43_238);
    }
    #[test]
    fn fen3_depth_5() {
        let mut p = Pos::from_str(P3_FEN).unwrap();
        assert_eq!(p.perft::<true>(5), 674_624);
    }
}
