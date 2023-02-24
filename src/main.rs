use std::str::FromStr;

use chess_lib::prelude::individual_squares::*;
use chess_lib::{evaluate, prelude::*, diagnostics};

use chess_lib::debug;

macro_rules! do_moves {
    ($P:ident $CNT:ident; $($F:ident $T:ident)*) => {
        $(
            $CNT -= 1;
            $P.do_move(Move::new($F, $T));
        )*
    }
}

// FIXME Position::do_move is slow
// FIXME Position::attacks_to_occ may be slow
// FIXME Position::compute_state and Position::generate_all are possibly slow

fn main() {
    initalize_magics();
    init();
    chess_lib::zobrist::init_zobrist();
    debug!("Done with initialization");

    let mut p = Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    println!("Nodes searched: {}", p.perft::<true>(7))
}
