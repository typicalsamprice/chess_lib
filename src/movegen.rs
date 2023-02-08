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

use crate::bitboard::Bitboard;
use crate::chessmove::{MType, Move};
use crate::color::Color;
use crate::filerank::{File, Rank};
use crate::init::{between, king_attack, knight_attack, pawn_attack};
use crate::magic::{bishop_moves, queen_moves, rook_moves};
use crate::piece::PType::{self, *};
use crate::position::{Castle, Position, State};
use crate::square::{individual_squares::*, Square};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GenType {
    Captures,
    Evasions,
    NonEvasions,
    Quiet,
    QuietChecks,
}

fn generate_pawn_moves(
    pos: &Position,
    list: &mut Vec<Move>,
    us: Color,
    gt: GenType,
    target: Bitboard,
) {
    let r3bb = Bitboard::from(Rank::Three.relative(us));
    let r7bb = Bitboard::from(Rank::Seven.relative(us));
    let pawns = pos.spec(Pawn, us);
    let on_7 = pawns & r7bb;
    let other = pawns ^ on_7;

    let enemies = if gt == GenType::Evasions {
        pos.state().checkers()
    } else {
        pos.color(!us)
    };
    let empty = !pos.all();

    let fw = Color::pawn_push(us);
    let back = Color::pawn_push(!us);

    if gt != GenType::Captures {
        let mut b1 = fw(other) & empty;
        let mut b2 = fw(b1 & r3bb) & empty;

        if gt == GenType::Evasions {
            b1 &= target;
            b2 &= target;
        }

        if gt == GenType::QuietChecks {
            let k = pos.king(!us);
            let dc_candidates = pos.state().blockers(!us).and_not(k.file());
            b1 &= pawn_attack(k, !us) | fw(dc_candidates);
            b2 &= pawn_attack(k, !us) | fw(fw(dc_candidates));
        }

        b1.map_by_board(|s| {
            let to = s.get_square();
            let from = back(s).get_square();
            list.push(Move::new(from, to));
        });
        b2.map_by_board(|s| {
            let to = s.get_square();
            let from = back(back(s)).get_square();
            list.push(Move::new(from, to));
        });
    }

    if on_7.nonzero() {
        let b1 = (fw(on_7) << 1).and_not(File::A) & enemies;
        let b2 = (fw(on_7) >> 1).and_not(File::H) & enemies;
        let mut b3 = fw(on_7) & empty;

        if gt == GenType::Evasions {
            b3 &= target;
        }

        let mut make_promos = |f: Square, t: Square| {
            list.push(Move::new(f, t).add_promo(Knight));
            list.push(Move::new(f, t).add_promo(Bishop));
            list.push(Move::new(f, t).add_promo(Rook));
            list.push(Move::new(f, t).add_promo(Queen));
        };

        b1.map_by_board(|s| {
            let from = (back(s) >> 1).get_square();
            let to = s.get_square();
            make_promos(from, to);
        });
        b2.map_by_board(|s| {
            let from = (back(s) << 1).get_square();
            let to = s.get_square();
            make_promos(from, to);
        });
        b3.map_by_board(|s| {
            let from = back(s).get_square();
            let to = s.get_square();
            make_promos(from, to);
        });
    }

    if gt == GenType::Captures || gt == GenType::Evasions || gt == GenType::NonEvasions {
        let b1 = (fw(other) << 1).and_not(File::A) & enemies;
        let b2 = (fw(other) >> 1).and_not(File::H) & enemies;

        b1.map_by_board(|s| {
            let to = s.get_square();
            let from = (back(s) >> 1).get_square();

            list.push(Move::new(from, to));
        });
        b2.map_by_board(|s| {
            let to = s.get_square();
            let from = (back(s) << 1).get_square();

            list.push(Move::new(from, to));
        });

        if pos.state().ep().is_ok() {
            let ep = pos.state().ep();
            debug_assert_eq!(ep.rank(), Rank::Six.relative(us));

            if gt == GenType::Evasions && (target & fw(Bitboard::from(ep))).nonzero() {
                return;
            }

            let mut b1 = other & pawn_attack(ep, !us);

            debug_assert!(b1.nonzero());

            while b1.nonzero() {
                let s = b1.get_square();
                b1 &= Bitboard::new(b1.inner() - 1);
                list.push(Move::new(s, ep).add_type(MType::EnPassant));
            }
        }
    }
}

fn generate_piece_moves(
    pos: &Position,
    list: &mut Vec<Move>,
    us: Color,
    gt: GenType,
    target: Bitboard,
    checks: bool,
) {
    for pt in [Knight, Bishop, Rook, Queen] {
        let pcs = pos.spec(pt, us);
        pcs.map_by_square(|s| {
            let mut b = gen_attacks(s, pt, pos.all(), pos.color(us)) & target;

            if checks && (pt == Queen || (pos.state().blockers(!us) & s).zero()) {
                b &= pos.state().check_squares(pt);
            }

            b.map_by_square(|d| {
                list.push(Move::new(s, d));
            });
        });
    }
}

fn gen_attacks(square: Square, pt: PType, occ: Bitboard, friendly: Bitboard) -> Bitboard {
    match pt {
        Pawn | King => panic!("Invalid piece type in `gen_attacks`"),
        Knight => knight_attack(square).and_not(friendly),
        Bishop => bishop_moves(square, occ).and_not(friendly),
        Rook => rook_moves(square, occ).and_not(friendly),
        Queen => {
            (bishop_moves(square, occ) | rook_moves(square, occ)).and_not(friendly)
        }
    }
}

fn generate_for(pos: &Position, list: &mut Vec<Move>, us: Color, gt: GenType) {
    let checks = gt == GenType::QuietChecks;
    let king = pos.king(us);
    let mut target = Bitboard::ZERO;
    if gt != GenType::Evasions || !pos.state().checkers().more_than_one() {
        target = if gt == GenType::Evasions {
            between::<true>(king, pos.state().checkers().get_square())
        } else if gt == GenType::NonEvasions {
            !pos.color(us)
        } else if gt == GenType::Captures {
            pos.color(!us)
        } else {
            !pos.all()
        };

        generate_pawn_moves(pos, list, us, gt, target);
        generate_piece_moves(pos, list, us, gt, target, checks);
    }

    if !checks || (pos.state().blockers(!us) & king).nonzero() {
        let mask = if gt == GenType::Evasions {
            !pos.color(us)
        } else {
            target
        };
        let mut b = king_attack(king) & mask;
        if checks {
            b &= !queen_moves(pos.king(!us), Bitboard::ZERO);
        }

        while b.nonzero() {
            let s = b.get_square();
            b &= Bitboard::new(b.inner() - 1);
            list.push(Move::new(king, s));
        }

        if gt == GenType::Quiet || gt == GenType::NonEvasions
        {
            let (ksc, qsc) = pos.state().cur_castle().castle_for(us);
            if ksc {
                let rk = G1.relative(us);
                let ib = between::<true>(king, rk);
                if (pos.all() & ib).zero() {
                    list.push(Move::new(king, G1.relative(us)).add_type(MType::Castle));
                }
            }
            if qsc {
                let rk = B1.relative(us);
                let ib = between::<true>(king, rk);
                if (pos.all() & ib).zero() {
                    list.push(Move::new(king, C1.relative(us)).add_type(MType::Castle));
                }
            }
        }
    }
}

pub fn generate_all(pos: &Position, list: &mut Vec<Move>, us: Color, gt: GenType) {
    debug_assert_eq!(gt == GenType::Evasions, pos.state().checkers().nonzero());
    generate_for(pos, list, us, gt);
}
pub fn generate_legal<const CLEAR_PREV: bool>(pos: &Position, list: &mut Vec<Move>) {
    let us = pos.to_move();
    if CLEAR_PREV {
        list.clear();
    }
    let gt = if pos.state().checkers().zero() {
        GenType::NonEvasions
    } else {
        GenType::Evasions
    };
    generate_all(pos, list, us, gt);

    let pinned = pos.state().blockers(us) & pos.color(us);
    let k = pos.king(us);

    list.retain(|&mv| {
        let should_check_legality = (pinned & mv.from()).nonzero() || mv.from() == k || mv.kind() == MType::EnPassant;
        !should_check_legality || pos.is_legal(mv)
    });
}
